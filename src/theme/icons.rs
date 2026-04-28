use ratatui::style::Color;

use crate::theme::palette::Palette;

/// Returns (icon, color) for a directory based on its name.
pub fn dir_icon(name: &str) -> (&'static str, Color) {
    match name.to_lowercase().as_str() {
        // Well-known directories with special icons
        ".git"           => ("\u{e5fb}", Palette::PEACH),     // 
        ".github"        => ("\u{e5fd}", Palette::SUBTEXT),   // 
        ".vscode"        => ("\u{e70c}", Palette::BLUE),      // 
        "node_modules"   => ("\u{e5fa}", Palette::GREEN),     // 
        "src"            => ("\u{f07c}", Palette::BLUE),      // 
        "bin"            => ("\u{f085}", Palette::GREEN),     // 
        "lib"            => ("\u{f121}", Palette::MAUVE),     // 
        "test" | "tests" => ("\u{f0c3}", Palette::YELLOW),   // 
        "docs" | "doc"   => ("\u{f02d}", Palette::TEAL),     // 
        "build" | "dist" | "target" => ("\u{f0ad}", Palette::PEACH), // 
        "config" | ".config"        => ("\u{e5fc}", Palette::DIM),   // 
        "assets" | "static" | "public" => ("\u{f03e}", Palette::PINK), // 
        "scripts"        => ("\u{f489}", Palette::GREEN),     // 
        "tmp" | "temp"   => ("\u{f2c2}", Palette::DIM),      // 
        "vendor"         => ("\u{f187}", Palette::DIM),       // 
        "downloads"      => ("\u{f019}", Palette::BLUE),     // 
        "documents"      => ("\u{f0f6}", Palette::TEAL),     // 
        "pictures" | "images" | "photos" => ("\u{f03e}", Palette::PINK), // 
        "music"          => ("\u{f001}", Palette::MAUVE),    // 
        "videos"         => ("\u{f03d}", Palette::RED),      // 
        "desktop"        => ("\u{f108}", Palette::BLUE),     // 
        _ => {
            // Default directory icons — open/closed style
            ("\u{f07b}", Palette::LAVENDER) //  — default folder
        }
    }
}

/// Returns (icon, color) for a symlink.
pub fn symlink_icon() -> (&'static str, Color) {
    ("\u{f0c1}", Palette::MAUVE) //  — link icon
}

/// Returns (icon, color) for a file based on its name and extension.
pub fn file_icon(name: &str, extension: &str) -> (&'static str, Color) {
    // Check exact filename matches first (higher priority)
    match name.to_lowercase().as_str() {
        "makefile"       => return ("\u{e673}", Palette::PEACH),
        "cmakelists.txt" => return ("\u{e673}", Palette::GREEN),
        "dockerfile"     => return ("\u{f308}", Palette::BLUE),
        "docker-compose.yml" | "docker-compose.yaml" | "compose.yml" | "compose.yaml"
                         => return ("\u{f308}", Palette::BLUE),
        ".gitignore"     => return ("\u{e5fb}", Palette::PEACH),
        ".gitmodules"    => return ("\u{e5fb}", Palette::PEACH),
        ".gitattributes" => return ("\u{e5fb}", Palette::PEACH),
        ".editorconfig"  => return ("\u{e615}", Palette::SUBTEXT),
        ".env" | ".env.local" | ".env.production"
                         => return ("\u{f462}", Palette::YELLOW),
        "license" | "licence" | "license.md" | "licence.md"
                         => return ("\u{f0219}", Palette::YELLOW),
        "readme.md" | "readme" | "readme.txt"
                         => return ("\u{f48a}", Palette::BLUE),
        "cargo.toml"     => return ("\u{e7a8}", Palette::PEACH),
        "cargo.lock"     => return ("\u{e7a8}", Palette::DIM),
        "package.json"   => return ("\u{e74e}", Palette::GREEN),
        "package-lock.json" | "yarn.lock" | "pnpm-lock.yaml"
                         => return ("\u{e74e}", Palette::DIM),
        "tsconfig.json"  => return ("\u{e628}", Palette::BLUE),
        "go.mod"         => return ("\u{e627}", Palette::TEAL),
        "go.sum"         => return ("\u{e627}", Palette::DIM),
        "flake.nix"      => return ("\u{f313}", Palette::SAPPHIRE),
        "flake.lock"     => return ("\u{f313}", Palette::DIM),
        _ => {}
    }

    // Extension-based lookup
    match extension {
        // ── Rust ────────────────────────────────
        "rs"        => ("\u{e7a8}", Palette::PEACH),
        "toml"      => ("\u{e615}", Palette::DIM),

        // ── Go ──────────────────────────────────
        "go"        => ("\u{e627}", Palette::TEAL),

        // ── Python ──────────────────────────────
        "py"        => ("\u{e73c}", Palette::YELLOW),
        "pyi"       => ("\u{e73c}", Palette::DIM),
        "pyc"       => ("\u{e73c}", Palette::DIM),
        "pyo"       => ("\u{e73c}", Palette::DIM),
        "ipynb"     => ("\u{e678}", Palette::PEACH),

        // ── JavaScript / TypeScript ─────────────
        "js"        => ("\u{e74e}", Palette::YELLOW),
        "mjs"       => ("\u{e74e}", Palette::YELLOW),
        "cjs"       => ("\u{e74e}", Palette::YELLOW),
        "jsx"       => ("\u{e7ba}", Palette::SAPPHIRE),
        "ts"        => ("\u{e628}", Palette::BLUE),
        "tsx"       => ("\u{e7ba}", Palette::BLUE),

        // ── Web ─────────────────────────────────
        "html" | "htm" => ("\u{e736}", Palette::PEACH),
        "css"       => ("\u{e749}", Palette::BLUE),
        "scss"      => ("\u{e749}", Palette::PINK),
        "sass"      => ("\u{e749}", Palette::PINK),
        "less"      => ("\u{e749}", Palette::MAUVE),
        "svg"       => ("\u{f1c5}", Palette::YELLOW),
        "wasm"      => ("\u{e6a1}", Palette::MAUVE),

        // ── C / C++ ─────────────────────────────
        "c"         => ("\u{e61e}", Palette::BLUE),
        "h"         => ("\u{e61e}", Palette::SUBTEXT),
        "cpp" | "cxx" | "cc" => ("\u{e61d}", Palette::BLUE),
        "hpp" | "hxx" | "hh" => ("\u{e61d}", Palette::SUBTEXT),

        // ── Java / Kotlin / Scala ───────────────
        "java"      => ("\u{e738}", Palette::RED),
        "kt" | "kts" => ("\u{e634}", Palette::MAUVE),
        "scala"     => ("\u{e737}", Palette::RED),
        "gradle"    => ("\u{e660}", Palette::TEAL),

        // ── Ruby ────────────────────────────────
        "rb"        => ("\u{e791}", Palette::RED),
        "erb"       => ("\u{e791}", Palette::RED),
        "gemspec"   => ("\u{e791}", Palette::RED),

        // ── PHP ─────────────────────────────────
        "php"       => ("\u{e73d}", Palette::MAUVE),

        // ── Swift / Objective-C ─────────────────
        "swift"     => ("\u{e755}", Palette::PEACH),
        "m"         => ("\u{e61e}", Palette::BLUE),

        // ── Shell ───────────────────────────────
        "sh" | "bash" | "zsh" | "fish" => ("\u{e795}", Palette::GREEN),
        "ps1"       => ("\u{e795}", Palette::BLUE),

        // ── Lua ─────────────────────────────────
        "lua"       => ("\u{e620}", Palette::BLUE),

        // ── Haskell / Elixir / Erlang ───────────
        "hs"        => ("\u{e777}", Palette::MAUVE),
        "ex" | "exs" => ("\u{e62d}", Palette::MAUVE),
        "erl"       => ("\u{e7b1}", Palette::RED),

        // ── Zig / Nim / V ───────────────────────
        "zig"       => ("\u{e6a9}", Palette::PEACH),
        "nim"       => ("\u{e677}", Palette::YELLOW),
        "v"         => ("\u{e6ac}", Palette::BLUE),

        // ── Data / Config ───────────────────────
        "json"      => ("\u{e60b}", Palette::YELLOW),
        "jsonc"     => ("\u{e60b}", Palette::YELLOW),
        "yaml" | "yml" => ("\u{e615}", Palette::RED),
        "xml"       => ("\u{e619}", Palette::PEACH),
        "csv"       => ("\u{f1c3}", Palette::GREEN),
        "sql"       => ("\u{e706}", Palette::BLUE),
        "graphql" | "gql" => ("\u{e662}", Palette::PINK),
        "proto"     => ("\u{e60b}", Palette::TEAL),
        "ini" | "cfg" => ("\u{e615}", Palette::DIM),

        // ── Markdown / Documentation ────────────
        "md" | "mdx" => ("\u{e73e}", Palette::BLUE),
        "rst"       => ("\u{f0f6}", Palette::TEAL),
        "txt"       => ("\u{f15c}", Palette::SUBTEXT),
        "tex" | "latex" => ("\u{e69b}", Palette::GREEN),
        "pdf"       => ("\u{f1c1}", Palette::RED),

        // ── Images ──────────────────────────────
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "ico"
                    => ("\u{f1c5}", Palette::PINK),
        "psd"       => ("\u{e7b8}", Palette::BLUE),
        "ai"        => ("\u{e7b4}", Palette::PEACH),

        // ── Video ───────────────────────────────
        "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "webm"
                    => ("\u{f03d}", Palette::RED),

        // ── Audio ───────────────────────────────
        "mp3" | "wav" | "flac" | "ogg" | "aac" | "m4a"
                    => ("\u{f001}", Palette::MAUVE),

        // ── Archives ────────────────────────────
        "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" | "zst"
                    => ("\u{f1c6}", Palette::YELLOW),
        "deb"       => ("\u{e77d}", Palette::RED),
        "rpm"       => ("\u{e7bb}", Palette::RED),

        // ── Executables & Libraries ─────────────
        "so" | "dylib" | "dll" => ("\u{f085}", Palette::DIM),
        "o" | "a"   => ("\u{f085}", Palette::DIM),
        "exe"       => ("\u{f085}", Palette::GREEN),

        // ── Nix ─────────────────────────────────
        "nix"       => ("\u{f313}", Palette::SAPPHIRE),

        // ── Docker ──────────────────────────────
        "containerfile" => ("\u{f308}", Palette::BLUE),

        // ── Terraform / DevOps ──────────────────
        "tf" | "hcl" => ("\u{e69a}", Palette::MAUVE),

        // ── Lock files ──────────────────────────
        "lock"      => ("\u{f023}", Palette::DIM),

        // ── Git ─────────────────────────────────
        "diff" | "patch" => ("\u{e5fb}", Palette::PEACH),

        // ── Certificates ────────────────────────
        "pem" | "crt" | "key" | "cer" => ("\u{f084}", Palette::YELLOW),

        // ── Default ─────────────────────────────
        _ => ("\u{f15c}", Palette::SUBTEXT), //  — generic file
    }
}
