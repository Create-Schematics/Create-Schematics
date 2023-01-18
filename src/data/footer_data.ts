interface FooterData {
    heading: string;
    body: string;
}

const FOOTER: FooterData[] = [
    {
        heading: 'About Us',
        body: '<p>If you have any questions, found any bugs or would like a new feature feel free to contact us either over discord, reddit or email</p><p>Discord: Rabbitminers#0086, Loading#0698</p>'
    },
    {
        heading: 'Other Projects',
        body: 'Create: Schematics is brought to you by the same team as the Create: Extended Series of mods and Create: Hand-Held-Contraptions, a demo site for which you can find here - '
    },
    {
        heading: 'Documentation',
        body: '<p>If you are interrested in creating your own addons for create we have written some hopefully helpful documentation to help introduce you to the kinetic system and the flywheel rendering engine, you can find this <a id="footer-link" href="/">Here</a><sup>WIP</sup></p>'
    }
]

export default FOOTER;