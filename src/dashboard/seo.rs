pub struct SeoMetadata<'a> {
    pub manifest: &'a str,
    pub favicon: Favicon<'a>,
    pub theme: ThemeColors<'a>,
    pub general: GeneralMetadata<'a>,
    pub twitter: Option<TwitterMetadata<'a>>,
    pub facebook: Option<FacebookMetadata<'a>>,
    pub schema_org: Option<&'a str>,
    pub article: Option<ArticleMetadata<'a>>,
}

impl<'a> SeoMetadata<'a> {
    pub fn new(
        manifest: &'a str,
        favicon: Favicon<'a>,
        theme: ThemeColors<'a>,
        general: GeneralMetadata<'a>,
    ) -> Self {
        SeoMetadata {
            manifest,
            favicon,
            theme,
            general,
            twitter: None,
            facebook: None,
            schema_org: None,
            article: None,
        }
    }
}

pub struct Favicon<'a> {
    pub icon: &'a str,
    pub apple_touch_icon: &'a str,
    pub icon_32x32: &'a str,
    pub icon_16x16: &'a str,
    pub mask_icon: &'a str,
}

impl<'a> Favicon<'a> {
    pub fn new(
        icon: &'a str,
        apple_touch_icon: &'a str,
        icon_32x32: &'a str,
        icon_16x16: &'a str,
        mask_icon: &'a str,
    ) -> Self {
        Favicon {
            icon,
            apple_touch_icon,
            icon_32x32,
            icon_16x16,
            mask_icon,
        }
    }
}

pub struct ThemeColors<'a> {
    pub mask_icon_color: &'a str,
    pub ms_tile_color: &'a str,
    pub theme_color: &'a str,
}

impl<'a> ThemeColors<'a> {
    pub fn new(mask_icon_color: &'a str, ms_tile_color: &'a str, theme_color: &'a str) -> Self {
        ThemeColors {
            mask_icon_color,
            ms_tile_color,
            theme_color,
        }
    }
}

pub struct GeneralMetadata<'a> {
    pub charset: &'a str,
    pub x_ua_compatible: &'a str,
    pub viewport: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub canonical_url: &'a str,
    pub keywords: Option<&'a str>,
    pub robots: Option<&'a str>,
    pub author: Option<&'a str>,
}

impl<'a> GeneralMetadata<'a> {
    pub fn new(
        charset: &'a str,
        x_ua_compatible: &'a str,
        viewport: &'a str,
        title: &'a str,
        description: &'a str,
        canonical_url: &'a str,
    ) -> Self {
        GeneralMetadata {
            charset,
            x_ua_compatible,
            viewport,
            title,
            description,
            canonical_url,
            keywords: None,
            robots: None,
            author: None,
        }
    }
}

pub struct TwitterMetadata<'a> {
    pub site: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub creator: &'a str,
    pub card: &'a str,
    pub image: &'a str,
}

impl<'a> TwitterMetadata<'a> {
    pub fn new(
        site: &'a str,
        title: &'a str,
        description: &'a str,
        creator: &'a str,
        card: &'a str,
        image: &'a str,
    ) -> Self {
        TwitterMetadata {
            site,
            title,
            description,
            creator,
            card,
            image,
        }
    }
}

pub struct FacebookMetadata<'a> {
    pub og_locale: &'a str,
    pub og_title: &'a str,
    pub og_url: &'a str,
    pub og_description: &'a str,
    pub og_site_name: &'a str,
    pub og_type: &'a str,
    pub og_image: &'a str,
}

impl<'a> FacebookMetadata<'a> {
    pub fn new(
        og_locale: &'a str,
        og_title: &'a str,
        og_url: &'a str,
        og_description: &'a str,
        og_site_name: &'a str,
        og_type: &'a str,
        og_image: &'a str,
    ) -> Self {
        FacebookMetadata {
            og_locale,
            og_title,
            og_url,
            og_description,
            og_site_name,
            og_type,
            og_image,
        }
    }
}

pub struct ArticleMetadata<'a> {
    pub author: &'a str,
    pub section: &'a str,
    pub tags: &'a str,
    pub published_time: &'a str,
    pub modified_time: &'a str,
}

impl<'a> ArticleMetadata<'a> {
    pub fn new(
        author: &'a str,
        section: &'a str,
        tags: &'a str,
        published_time: &'a str,
        modified_time: &'a str,
    ) -> Self {
        ArticleMetadata {
            author,
            section,
            tags,
            published_time,
            modified_time,
        }
    }
}

fn main() {
    let favicon = Favicon::new(
        "/favicon.ico",
        "/apple-touch-icon.png",
        "/favicon-32x32.png",
        "/favicon-16x16.png",
        "/mask-icon.svg",
    );
    let theme = ThemeColors::new("#ffffff", "#ffffff", "#ffffff");
    let general = GeneralMetadata::new(
        "UTF-8",
        "IE=edge",
        "width=device-width, initial-scale=1, shrink-to-fit=no",
        "Page Title",
        "Description",
        "https://example.com",
    );

    let twitter = Some(TwitterMetadata::new(
        "@twitter",
        "Page Title",
        "Description",
        "@creator",
        "summary",
        "https://example.com/image.png",
    ));

    let facebook = Some(FacebookMetadata::new(
        "es",
        "Page Title",
        "https://example.com",
        "Description",
        "Site Name",
        "website",
        "https://example.com/image.png",
    ));

    let seo_metadata = SeoMetadata {
        manifest: "/manifest.json",
        favicon,
        theme,
        general,
        twitter,
        facebook,
        schema_org: Some("{}"),
        article: None,
    };
}
