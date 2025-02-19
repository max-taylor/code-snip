use std::str::FromStr;

use syntect::highlighting::{
    Color, ScopeSelector, ScopeSelectors, StyleModifier, Theme, ThemeItem, ThemeSettings,
};
use syntect::parsing::ScopeStack;

pub fn build_github_dark_theme() -> Theme {
    // Define the theme settings
    let settings = ThemeSettings {
        foreground: Some(Color {
            r: 201,
            g: 209,
            b: 217,
            a: 255,
        }),
        background: Some(Color {
            r: 13,
            g: 17,
            b: 23,
            a: 255,
        }),
        caret: Some(Color {
            r: 201,
            g: 209,
            b: 217,
            a: 255,
        }),
        line_highlight: Some(Color {
            r: 13,
            g: 17,
            b: 23,
            a: 48,
        }),
        selection: Some(Color {
            r: 33,
            g: 38,
            b: 45,
            a: 255,
        }),
        misspelling: None,
        minimap_border: None,
        accent: None,
        popup_css: None,
        phantom_css: None,
        bracket_contents_foreground: None,
        bracket_contents_options: None,
        brackets_foreground: None,
        brackets_background: None,
        brackets_options: None,
        tags_foreground: None,
        tags_options: None,
        highlight: None,
        find_highlight: None,
        find_highlight_foreground: None,
        gutter: None,
        gutter_foreground: None,
        selection_foreground: None,
        selection_border: None,
        inactive_selection: None,
        inactive_selection_foreground: None,
        guide: None,
        active_guide: None,
        stack_guide: None,
        shadow: None,
    };

    // Define the theme items (scopes)
    let scopes = vec![
        // Comments
        ThemeItem {
            scope: ScopeSelectors {
                selectors: vec![
                    ScopeSelector {
                        path: ScopeStack::from_str("comment").unwrap(),
                        excludes: vec![],
                    },
                    ScopeSelector {
                        path: ScopeStack::from_str("punctuation.definition.comment").unwrap(),
                        excludes: vec![],
                    },
                ],
            },
            style: StyleModifier {
                foreground: Some(Color {
                    r: 106,
                    g: 115,
                    b: 125,
                    a: 255,
                }),
                background: None,
                font_style: None,
            },
        },
        // Keywords
        ThemeItem {
            scope: ScopeSelectors {
                selectors: vec![ScopeSelector {
                    path: ScopeStack::from_str("keyword").unwrap(),
                    excludes: vec![],
                }],
            },
            style: StyleModifier {
                foreground: Some(Color {
                    r: 255,
                    g: 123,
                    b: 114,
                    a: 255,
                }),
                background: None,
                font_style: None,
            },
        },
        // Variables
        ThemeItem {
            scope: ScopeSelectors {
                selectors: vec![
                    ScopeSelector {
                        path: ScopeStack::from_str("variable").unwrap(),
                        excludes: vec![],
                    },
                    ScopeSelector {
                        path: ScopeStack::from_str("variable.other.dollar.only.js").unwrap(),
                        excludes: vec![],
                    },
                ],
            },
            style: StyleModifier {
                foreground: Some(Color {
                    r: 121,
                    g: 192,
                    b: 255,
                    a: 255,
                }),
                background: None,
                font_style: None,
            },
        },
        // Functions
        ThemeItem {
            scope: ScopeSelectors {
                selectors: vec![
                    ScopeSelector {
                        path: ScopeStack::from_str("entity.name.function").unwrap(),
                        excludes: vec![],
                    },
                    ScopeSelector {
                        path: ScopeStack::from_str("meta.require").unwrap(),
                        excludes: vec![],
                    },
                    ScopeSelector {
                        path: ScopeStack::from_str("support.function.any-method").unwrap(),
                        excludes: vec![],
                    },
                    ScopeSelector {
                        path: ScopeStack::from_str("variable.function").unwrap(),
                        excludes: vec![],
                    },
                ],
            },
            style: StyleModifier {
                foreground: Some(Color {
                    r: 121,
                    g: 192,
                    b: 255,
                    a: 255,
                }),
                background: None,
                font_style: None,
            },
        },
        // Classes
        ThemeItem {
            scope: ScopeSelectors {
                selectors: vec![
                    ScopeSelector {
                        path: ScopeStack::from_str("support.class").unwrap(),
                        excludes: vec![],
                    },
                    ScopeSelector {
                        path: ScopeStack::from_str("entity.name.class").unwrap(),
                        excludes: vec![],
                    },
                    ScopeSelector {
                        path: ScopeStack::from_str("entity.name.type.class").unwrap(),
                        excludes: vec![],
                    },
                ],
            },
            style: StyleModifier {
                foreground: Some(Color {
                    r: 255,
                    g: 166,
                    b: 87,
                    a: 255,
                }),
                background: None,
                font_style: None,
            },
        },
        // Strings
        ThemeItem {
            scope: ScopeSelectors {
                selectors: vec![
                    ScopeSelector {
                        path: ScopeStack::from_str("string").unwrap(),
                        excludes: vec![],
                    },
                    ScopeSelector {
                        path: ScopeStack::from_str("constant.other.symbol").unwrap(),
                        excludes: vec![],
                    },
                    ScopeSelector {
                        path: ScopeStack::from_str("entity.other.inherited-class").unwrap(),
                        excludes: vec![],
                    },
                ],
            },
            style: StyleModifier {
                foreground: Some(Color {
                    r: 165,
                    g: 214,
                    b: 132,
                    a: 255,
                }),
                background: None,
                font_style: None,
            },
        },
        // Numeric constants
        ThemeItem {
            scope: ScopeSelectors {
                selectors: vec![ScopeSelector {
                    path: ScopeStack::from_str("constant.numeric").unwrap(),
                    excludes: vec![],
                }],
            },
            style: StyleModifier {
                foreground: Some(Color {
                    r: 249,
                    g: 145,
                    b: 87,
                    a: 255,
                }),
                background: None,
                font_style: None,
            },
        },
        // Tags
        ThemeItem {
            scope: ScopeSelectors {
                selectors: vec![ScopeSelector {
                    path: ScopeStack::from_str("entity.name.tag").unwrap(),
                    excludes: vec![],
                }],
            },
            style: StyleModifier {
                foreground: Some(Color {
                    r: 121,
                    g: 192,
                    b: 255,
                    a: 255,
                }),
                background: None,
                font_style: None,
            },
        },
        // Attributes
        ThemeItem {
            scope: ScopeSelectors {
                selectors: vec![ScopeSelector {
                    path: ScopeStack::from_str("entity.other.attribute-name").unwrap(),
                    excludes: vec![],
                }],
            },
            style: StyleModifier {
                foreground: Some(Color {
                    r: 249,
                    g: 145,
                    b: 87,
                    a: 255,
                }),
                background: None,
                font_style: None,
            },
        },
        // Headings
        ThemeItem {
            scope: ScopeSelectors {
                selectors: vec![
                    ScopeSelector {
                        path: ScopeStack::from_str("markup.heading").unwrap(),
                        excludes: vec![],
                    },
                    ScopeSelector {
                        path: ScopeStack::from_str("punctuation.definition.heading").unwrap(),
                        excludes: vec![],
                    },
                    ScopeSelector {
                        path: ScopeStack::from_str("entity.name.section").unwrap(),
                        excludes: vec![],
                    },
                ],
            },
            style: StyleModifier {
                foreground: Some(Color {
                    r: 121,
                    g: 192,
                    b: 255,
                    a: 255,
                }),
                background: None,
                font_style: None,
            },
        },
        // Bold text
        ThemeItem {
            scope: ScopeSelectors {
                selectors: vec![
                    ScopeSelector {
                        path: ScopeStack::from_str("markup.bold").unwrap(),
                        excludes: vec![],
                    },
                    ScopeSelector {
                        path: ScopeStack::from_str("punctuation.definition.bold").unwrap(),
                        excludes: vec![],
                    },
                ],
            },
            style: StyleModifier {
                foreground: Some(Color {
                    r: 255,
                    g: 166,
                    b: 87,
                    a: 255,
                }),
                background: None,
                font_style: Some(syntect::highlighting::FontStyle::BOLD),
            },
        },
        // Italic text
        ThemeItem {
            scope: ScopeSelectors {
                selectors: vec![
                    ScopeSelector {
                        path: ScopeStack::from_str("markup.italic").unwrap(),
                        excludes: vec![],
                    },
                    ScopeSelector {
                        path: ScopeStack::from_str("punctuation.definition.italic").unwrap(),
                        excludes: vec![],
                    },
                ],
            },
            style: StyleModifier {
                foreground: Some(Color {
                    r: 255,
                    g: 123,
                    b: 114,
                    a: 255,
                }),
                background: None,
                font_style: Some(syntect::highlighting::FontStyle::ITALIC),
            },
        },
        // Inline raw text
        ThemeItem {
            scope: ScopeSelectors {
                selectors: vec![ScopeSelector {
                    path: ScopeStack::from_str("markup.raw.inline").unwrap(),
                    excludes: vec![],
                }],
            },
            style: StyleModifier {
                foreground: Some(Color {
                    r: 165,
                    g: 214,
                    b: 132,
                    a: 255,
                }),
                background: None,
                font_style: None,
            },
        },
    ];

    // Create and return the theme
    Theme {
        name: Some("GitHub Dark".to_string()),
        author: Some("GitHub".to_string()),
        settings,
        scopes,
    }
}
