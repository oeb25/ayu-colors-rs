pub mod dark {
    pub mod syntax {
        use crate::Shade;
        /// `fg=#39bae6` `bg=#0d1017`
        pub const tag: Shade = Shade::new([57, 186, 230, 255], [13, 16, 23, 255]);
        /// `fg=#ffb454` `bg=#0d1017`
        pub const func: Shade = Shade::new([255, 180, 84, 255], [13, 16, 23, 255]);
        /// `fg=#59c2ff` `bg=#0d1017`
        pub const entity: Shade = Shade::new([89, 194, 255, 255], [13, 16, 23, 255]);
        /// `fg=#aad94c` `bg=#0d1017`
        pub const string: Shade = Shade::new([170, 217, 76, 255], [13, 16, 23, 255]);
        /// `fg=#95e6cb` `bg=#0d1017`
        pub const regexp: Shade = Shade::new([149, 230, 203, 255], [13, 16, 23, 255]);
        /// `fg=#f07178` `bg=#0d1017`
        pub const markup: Shade = Shade::new([240, 113, 120, 255], [13, 16, 23, 255]);
        /// `fg=#ff8f40` `bg=#0d1017`
        pub const keyword: Shade = Shade::new([255, 143, 64, 255], [13, 16, 23, 255]);
        /// `fg=#e6b673` `bg=#0d1017`
        pub const special: Shade = Shade::new([230, 182, 115, 255], [13, 16, 23, 255]);
        /// `fg=#acb6bf8c` `bg=#0d1017`
        pub const comment: Shade = Shade::new([172, 182, 191, 140], [13, 16, 23, 255]);
        /// `fg=#d2a6ff` `bg=#0d1017`
        pub const constant: Shade = Shade::new([210, 166, 255, 255], [13, 16, 23, 255]);
        /// `fg=#f29668` `bg=#0d1017`
        pub const operator: Shade = Shade::new([242, 150, 104, 255], [13, 16, 23, 255]);
    }
    pub mod vcs {
        use crate::Shade;
        /// `fg=#7fd962` `bg=#0d1017`
        pub const added: Shade = Shade::new([127, 217, 98, 255], [13, 16, 23, 255]);
        /// `fg=#73b8ff` `bg=#0d1017`
        pub const modified: Shade = Shade::new([115, 184, 255, 255], [13, 16, 23, 255]);
        /// `fg=#f26d78` `bg=#0d1017`
        pub const removed: Shade = Shade::new([242, 109, 120, 255], [13, 16, 23, 255]);
    }
    pub mod editor {
        use crate::Shade;
        /// `fg=#bfbdb6` `bg=#0d1017`
        pub const fg: Shade = Shade::new([191, 189, 182, 255], [13, 16, 23, 255]);
        /// `fg=#0d1017` `bg=#0d1017`
        pub const bg: Shade = Shade::new([13, 16, 23, 255], [13, 16, 23, 255]);
        /// `fg=#131721` `bg=#0d1017`
        pub const line: Shade = Shade::new([19, 23, 33, 255], [13, 16, 23, 255]);
        pub mod selection {
            use crate::Shade;
            /// `fg=#409fff4d` `bg=#0d1017`
            pub const active: Shade = Shade::new([64, 159, 255, 77], [13, 16, 23, 255]);
            /// `fg=#409fff21` `bg=#0d1017`
            pub const inactive: Shade = Shade::new([64, 159, 255, 33], [13, 16, 23, 255]);
        }
        pub mod find_match {
            use crate::Shade;
            /// `fg=#6c5980` `bg=#0d1017`
            pub const active: Shade = Shade::new([108, 89, 128, 255], [13, 16, 23, 255]);
            /// `fg=#6c598066` `bg=#0d1017`
            pub const inactive: Shade = Shade::new([108, 89, 128, 102], [13, 16, 23, 255]);
        }
        pub mod gutter {
            use crate::Shade;
            /// `fg=#6c7380e6` `bg=#0d1017`
            pub const active: Shade = Shade::new([108, 115, 128, 230], [13, 16, 23, 255]);
            /// `fg=#6c738099` `bg=#0d1017`
            pub const normal: Shade = Shade::new([108, 115, 128, 153], [13, 16, 23, 255]);
        }
        pub mod indent_guide {
            use crate::Shade;
            /// `fg=#6c738080` `bg=#0d1017`
            pub const active: Shade = Shade::new([108, 115, 128, 128], [13, 16, 23, 255]);
            /// `fg=#6c738033` `bg=#0d1017`
            pub const normal: Shade = Shade::new([108, 115, 128, 51], [13, 16, 23, 255]);
        }
    }
    pub mod ui {
        use crate::Shade;
        /// `fg=#565b66` `bg=#0b0e14`
        pub const fg: Shade = Shade::new([86, 91, 102, 255], [11, 14, 20, 255]);
        /// `fg=#0b0e14` `bg=#0b0e14`
        pub const bg: Shade = Shade::new([11, 14, 20, 255], [11, 14, 20, 255]);
        /// `fg=#11151c` `bg=#0b0e14`
        pub const line: Shade = Shade::new([17, 21, 28, 255], [11, 14, 20, 255]);
        pub mod selection {
            use crate::Shade;
            /// `fg=#47526640` `bg=#0b0e14`
            pub const active: Shade = Shade::new([71, 82, 102, 64], [11, 14, 20, 255]);
            /// `fg=#47526633` `bg=#0b0e14`
            pub const normal: Shade = Shade::new([71, 82, 102, 51], [11, 14, 20, 255]);
        }
        pub mod panel {
            use crate::Shade;
            /// `fg=#0f131a` `bg=#0b0e14`
            pub const bg: Shade = Shade::new([15, 19, 26, 255], [11, 14, 20, 255]);
            /// `fg=#00000080` `bg=#0b0e14`
            pub const shadow: Shade = Shade::new([0, 0, 0, 128], [11, 14, 20, 255]);
        }
    }
    pub mod common {
        use crate::Shade;
        /// `fg=#e6b450` `bg=#0b0e14`
        pub const accent: Shade = Shade::new([230, 180, 80, 255], [11, 14, 20, 255]);
        /// `fg=#d95757` `bg=#0b0e14`
        pub const error: Shade = Shade::new([217, 87, 87, 255], [11, 14, 20, 255]);
    }
}
pub mod light {
    pub mod syntax {
        use crate::Shade;
        /// `fg=#55b4d4` `bg=#fcfcfc`
        pub const tag: Shade = Shade::new([85, 180, 212, 255], [252, 252, 252, 255]);
        /// `fg=#f2ae49` `bg=#fcfcfc`
        pub const func: Shade = Shade::new([242, 174, 73, 255], [252, 252, 252, 255]);
        /// `fg=#399ee6` `bg=#fcfcfc`
        pub const entity: Shade = Shade::new([57, 158, 230, 255], [252, 252, 252, 255]);
        /// `fg=#86b300` `bg=#fcfcfc`
        pub const string: Shade = Shade::new([134, 179, 0, 255], [252, 252, 252, 255]);
        /// `fg=#4cbf99` `bg=#fcfcfc`
        pub const regexp: Shade = Shade::new([76, 191, 153, 255], [252, 252, 252, 255]);
        /// `fg=#f07171` `bg=#fcfcfc`
        pub const markup: Shade = Shade::new([240, 113, 113, 255], [252, 252, 252, 255]);
        /// `fg=#fa8d3e` `bg=#fcfcfc`
        pub const keyword: Shade = Shade::new([250, 141, 62, 255], [252, 252, 252, 255]);
        /// `fg=#e6ba7e` `bg=#fcfcfc`
        pub const special: Shade = Shade::new([230, 186, 126, 255], [252, 252, 252, 255]);
        /// `fg=#787b8099` `bg=#fcfcfc`
        pub const comment: Shade = Shade::new([120, 123, 128, 153], [252, 252, 252, 255]);
        /// `fg=#a37acc` `bg=#fcfcfc`
        pub const constant: Shade = Shade::new([163, 122, 204, 255], [252, 252, 252, 255]);
        /// `fg=#ed9366` `bg=#fcfcfc`
        pub const operator: Shade = Shade::new([237, 147, 102, 255], [252, 252, 252, 255]);
    }
    pub mod vcs {
        use crate::Shade;
        /// `fg=#6cbf43` `bg=#fcfcfc`
        pub const added: Shade = Shade::new([108, 191, 67, 255], [252, 252, 252, 255]);
        /// `fg=#478acc` `bg=#fcfcfc`
        pub const modified: Shade = Shade::new([71, 138, 204, 255], [252, 252, 252, 255]);
        /// `fg=#ff7383` `bg=#fcfcfc`
        pub const removed: Shade = Shade::new([255, 115, 131, 255], [252, 252, 252, 255]);
    }
    pub mod editor {
        use crate::Shade;
        /// `fg=#5c6166` `bg=#fcfcfc`
        pub const fg: Shade = Shade::new([92, 97, 102, 255], [252, 252, 252, 255]);
        /// `fg=#fcfcfc` `bg=#fcfcfc`
        pub const bg: Shade = Shade::new([252, 252, 252, 255], [252, 252, 252, 255]);
        /// `fg=#8a91991a` `bg=#fcfcfc`
        pub const line: Shade = Shade::new([138, 145, 153, 26], [252, 252, 252, 255]);
        pub mod selection {
            use crate::Shade;
            /// `fg=#035bd626` `bg=#fcfcfc`
            pub const active: Shade = Shade::new([3, 91, 214, 38], [252, 252, 252, 255]);
            /// `fg=#035bd612` `bg=#fcfcfc`
            pub const inactive: Shade = Shade::new([3, 91, 214, 18], [252, 252, 252, 255]);
        }
        pub mod find_match {
            use crate::Shade;
            /// `fg=#9f40ff2b` `bg=#fcfcfc`
            pub const active: Shade = Shade::new([159, 64, 255, 43], [252, 252, 252, 255]);
            /// `fg=#9f40ffcc` `bg=#fcfcfc`
            pub const inactive: Shade = Shade::new([159, 64, 255, 204], [252, 252, 252, 255]);
        }
        pub mod gutter {
            use crate::Shade;
            /// `fg=#8a9199cc` `bg=#fcfcfc`
            pub const active: Shade = Shade::new([138, 145, 153, 204], [252, 252, 252, 255]);
            /// `fg=#8a919966` `bg=#fcfcfc`
            pub const normal: Shade = Shade::new([138, 145, 153, 102], [252, 252, 252, 255]);
        }
        pub mod indent_guide {
            use crate::Shade;
            /// `fg=#8a919959` `bg=#fcfcfc`
            pub const active: Shade = Shade::new([138, 145, 153, 89], [252, 252, 252, 255]);
            /// `fg=#8a91992e` `bg=#fcfcfc`
            pub const normal: Shade = Shade::new([138, 145, 153, 46], [252, 252, 252, 255]);
        }
    }
    pub mod ui {
        use crate::Shade;
        /// `fg=#8a9199` `bg=#f8f9fa`
        pub const fg: Shade = Shade::new([138, 145, 153, 255], [248, 249, 250, 255]);
        /// `fg=#f8f9fa` `bg=#f8f9fa`
        pub const bg: Shade = Shade::new([248, 249, 250, 255], [248, 249, 250, 255]);
        /// `fg=#6b7d8f1f` `bg=#f8f9fa`
        pub const line: Shade = Shade::new([107, 125, 143, 31], [248, 249, 250, 255]);
        pub mod selection {
            use crate::Shade;
            /// `fg=#56728f1f` `bg=#f8f9fa`
            pub const active: Shade = Shade::new([86, 114, 143, 31], [248, 249, 250, 255]);
            /// `fg=#6b7d8f1f` `bg=#f8f9fa`
            pub const normal: Shade = Shade::new([107, 125, 143, 31], [248, 249, 250, 255]);
        }
        pub mod panel {
            use crate::Shade;
            /// `fg=#f3f4f5` `bg=#f8f9fa`
            pub const bg: Shade = Shade::new([243, 244, 245, 255], [248, 249, 250, 255]);
            /// `fg=#00000026` `bg=#f8f9fa`
            pub const shadow: Shade = Shade::new([0, 0, 0, 38], [248, 249, 250, 255]);
        }
    }
    pub mod common {
        use crate::Shade;
        /// `fg=#ffaa33` `bg=#f8f9fa`
        pub const accent: Shade = Shade::new([255, 170, 51, 255], [248, 249, 250, 255]);
        /// `fg=#e65050` `bg=#f8f9fa`
        pub const error: Shade = Shade::new([230, 80, 80, 255], [248, 249, 250, 255]);
    }
}
pub mod mirage {
    pub mod syntax {
        use crate::Shade;
        /// `fg=#5ccfe6` `bg=#242936`
        pub const tag: Shade = Shade::new([92, 207, 230, 255], [36, 41, 54, 255]);
        /// `fg=#ffd173` `bg=#242936`
        pub const func: Shade = Shade::new([255, 209, 115, 255], [36, 41, 54, 255]);
        /// `fg=#73d0ff` `bg=#242936`
        pub const entity: Shade = Shade::new([115, 208, 255, 255], [36, 41, 54, 255]);
        /// `fg=#d5ff80` `bg=#242936`
        pub const string: Shade = Shade::new([213, 255, 128, 255], [36, 41, 54, 255]);
        /// `fg=#95e6cb` `bg=#242936`
        pub const regexp: Shade = Shade::new([149, 230, 203, 255], [36, 41, 54, 255]);
        /// `fg=#f28779` `bg=#242936`
        pub const markup: Shade = Shade::new([242, 135, 121, 255], [36, 41, 54, 255]);
        /// `fg=#ffad66` `bg=#242936`
        pub const keyword: Shade = Shade::new([255, 173, 102, 255], [36, 41, 54, 255]);
        /// `fg=#ffdfb3` `bg=#242936`
        pub const special: Shade = Shade::new([255, 223, 179, 255], [36, 41, 54, 255]);
        /// `fg=#b8cfe680` `bg=#242936`
        pub const comment: Shade = Shade::new([184, 207, 230, 128], [36, 41, 54, 255]);
        /// `fg=#dfbfff` `bg=#242936`
        pub const constant: Shade = Shade::new([223, 191, 255, 255], [36, 41, 54, 255]);
        /// `fg=#f29e74` `bg=#242936`
        pub const operator: Shade = Shade::new([242, 158, 116, 255], [36, 41, 54, 255]);
    }
    pub mod vcs {
        use crate::Shade;
        /// `fg=#87d96c` `bg=#242936`
        pub const added: Shade = Shade::new([135, 217, 108, 255], [36, 41, 54, 255]);
        /// `fg=#80bfff` `bg=#242936`
        pub const modified: Shade = Shade::new([128, 191, 255, 255], [36, 41, 54, 255]);
        /// `fg=#f27983` `bg=#242936`
        pub const removed: Shade = Shade::new([242, 121, 131, 255], [36, 41, 54, 255]);
    }
    pub mod editor {
        use crate::Shade;
        /// `fg=#cccac2` `bg=#242936`
        pub const fg: Shade = Shade::new([204, 202, 194, 255], [36, 41, 54, 255]);
        /// `fg=#242936` `bg=#242936`
        pub const bg: Shade = Shade::new([36, 41, 54, 255], [36, 41, 54, 255]);
        /// `fg=#1a1f29` `bg=#242936`
        pub const line: Shade = Shade::new([26, 31, 41, 255], [36, 41, 54, 255]);
        pub mod selection {
            use crate::Shade;
            /// `fg=#409fff40` `bg=#242936`
            pub const active: Shade = Shade::new([64, 159, 255, 64], [36, 41, 54, 255]);
            /// `fg=#409fff21` `bg=#242936`
            pub const inactive: Shade = Shade::new([64, 159, 255, 33], [36, 41, 54, 255]);
        }
        pub mod find_match {
            use crate::Shade;
            /// `fg=#695380` `bg=#242936`
            pub const active: Shade = Shade::new([105, 83, 128, 255], [36, 41, 54, 255]);
            /// `fg=#69538066` `bg=#242936`
            pub const inactive: Shade = Shade::new([105, 83, 128, 102], [36, 41, 54, 255]);
        }
        pub mod gutter {
            use crate::Shade;
            /// `fg=#8a9199cc` `bg=#242936`
            pub const active: Shade = Shade::new([138, 145, 153, 204], [36, 41, 54, 255]);
            /// `fg=#8a919966` `bg=#242936`
            pub const normal: Shade = Shade::new([138, 145, 153, 102], [36, 41, 54, 255]);
        }
        pub mod indent_guide {
            use crate::Shade;
            /// `fg=#8a919959` `bg=#242936`
            pub const active: Shade = Shade::new([138, 145, 153, 89], [36, 41, 54, 255]);
            /// `fg=#8a91992e` `bg=#242936`
            pub const normal: Shade = Shade::new([138, 145, 153, 46], [36, 41, 54, 255]);
        }
    }
    pub mod ui {
        use crate::Shade;
        /// `fg=#707a8c` `bg=#1f2430`
        pub const fg: Shade = Shade::new([112, 122, 140, 255], [31, 36, 48, 255]);
        /// `fg=#1f2430` `bg=#1f2430`
        pub const bg: Shade = Shade::new([31, 36, 48, 255], [31, 36, 48, 255]);
        /// `fg=#171b24` `bg=#1f2430`
        pub const line: Shade = Shade::new([23, 27, 36, 255], [31, 36, 48, 255]);
        pub mod selection {
            use crate::Shade;
            /// `fg=#63759926` `bg=#1f2430`
            pub const active: Shade = Shade::new([99, 117, 153, 38], [31, 36, 48, 255]);
            /// `fg=#69758c1f` `bg=#1f2430`
            pub const normal: Shade = Shade::new([105, 117, 140, 31], [31, 36, 48, 255]);
        }
        pub mod panel {
            use crate::Shade;
            /// `fg=#1c212b` `bg=#1f2430`
            pub const bg: Shade = Shade::new([28, 33, 43, 255], [31, 36, 48, 255]);
            /// `fg=#12151cb3` `bg=#1f2430`
            pub const shadow: Shade = Shade::new([18, 21, 28, 179], [31, 36, 48, 255]);
        }
    }
    pub mod common {
        use crate::Shade;
        /// `fg=#ffcc66` `bg=#1f2430`
        pub const accent: Shade = Shade::new([255, 204, 102, 255], [31, 36, 48, 255]);
        /// `fg=#ff6666` `bg=#1f2430`
        pub const error: Shade = Shade::new([255, 102, 102, 255], [31, 36, 48, 255]);
    }
}
