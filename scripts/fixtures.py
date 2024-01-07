import argparse
import psycopg2
import os

from faker import Faker
from dotenv import load_dotenv

load_dotenv()

Conn = psycopg2.extensions.connection

def read_env(key: str) -> str:
    value = os.getenv(key)

    if value is None:
        raise ValueError(f'Missing environment variable: {key}')
    else:
        return value

DATABASE_URL: str = read_env('DATABASE_URL')
FAKE = Faker()

def create_schematics(connection: Conn, user: str, count: int) -> None:
    cursor = connection.cursor()

    args = b','.join(cursor.mogrify("(%s, %s, %s, %s, %s, %s, %s)", mock_schematic(user)) for _ in range(count))
    cursor.execute(b"INSERT INTO schematics (schematic_name, body, author, images, files, game_version_id, create_version_id) VALUES " + args)

    connection.commit()
    cursor.close()

def mock_schematic(user: str) -> tuple:
    return (FAKE.word(), FAKE.paragraph(), user, [], [], 5, 8)

def create_users(connection: Conn, count: int) -> list[tuple[str]]:
    cursor = connection.cursor()

    args = b','.join(cursor.mogrify("(%s, %s, %s, %s, %s)", mock_user()) for _ in range(count))
    cursor.execute(b"INSERT INTO users (displayname, username, email, oauth_id, oauth_provider) VALUES " + args + b" RETURNING user_id")

    result = cursor.fetchall()

    connection.commit()
    cursor.close()
    
    return result # type: ignore

def mock_user() -> tuple:
    name = FAKE.name()

    return (name, snake_case(name), FAKE.email(), FAKE.ssn(), "fixture")

def connect(database_url: str = DATABASE_URL) -> Conn:
    try:
        connection = psycopg2.connect(database_url)

        return connection
    except (Exception, psycopg2.Error) as error:
        print('Error while connecting to PostgreSQL', error)
        exit()

def snake_case(input_string):
    return input_string.replace(' ', '_').replace('-', '_').lower()


def main(args) -> None:
    connection = connect()

    user_count = args.users or 5
    users = create_users(connection, user_count)

    schematic_count = args.schematics or 5

    for user in users:
        create_schematics(connection, user[0], schematic_count)

    connection.close()

if __name__ == '__main__':
    parser = argparse.ArgumentParser()

    parser.add_argument('-u', '--users', type=int, help='The number of mock users to create')
    parser.add_argument('-s', '--schematics', type=int, help='The number of mock schematics to create on each user')

    args = parser.parse_args()

    main(args)