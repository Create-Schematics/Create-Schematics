use std::time::Duration;

use poem::web::cookie::{SameSite, Cookie};

pub struct CookieBuilder {
    name: String,
    value: String,
    path: String,
    secure: bool,
    http_only: bool,
    max_age: Option<Duration>,
    same_site: Option<SameSite>,
}

impl CookieBuilder {
    pub fn new<K, V>(name: K, value: V) -> Self 
    where
        K: Into<String>,
        V: Into<String>
    {
        Self {
            name: name.into(),
            value: value.into(),
            path: "/".into(),
            secure: true,
            http_only: true,
            max_age: None,
            same_site: None,
        }
    }

    pub fn path<T>(self, value: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            path: value.into(), 
            ..self
        }
    }

    pub fn max_age(self, value: Duration) -> Self {
        Self {
            max_age: Some(value),
            ..self
        }
    }

    pub fn same_site(self, value: SameSite) -> Self {
        Self {
            same_site: Some(value),
            ..self
        }
    }

    pub fn secure(self, value: bool) -> Self {
        Self {
            secure: value,
            ..self
        }
    }

    pub fn http_only(self, value: bool) -> Self {
        Self {
            http_only: value,
            ..self
        }
    }

    pub fn build(self) -> Cookie {
        let mut cookie = Cookie::new_with_str(&self.name, self.value);

        cookie.set_secure(self.secure);
        cookie.set_http_only(self.http_only);

        cookie.set_path(&self.path);

        if let Some(max_age) = &self.max_age {
            cookie.set_max_age(*max_age);
        }

        cookie.set_same_site(self.same_site);

        cookie
    }
}