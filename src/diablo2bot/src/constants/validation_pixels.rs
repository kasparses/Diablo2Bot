use crate::{point_u16::PointU16, state_validator::ValidationPixel, structs::Pixel};

pub const AUTOMAP_SHOW_PARTY_NO_VALIDATION_PIXELS: [ValidationPixel; 7] = [
    ValidationPixel {
        point: PointU16 { row: 311, col: 626 },
        pixel: Pixel {
            red: 28,
            green: 24,
            blue: 8,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 296, col: 617 },
        pixel: Pixel {
            red: 40,
            green: 36,
            blue: 28,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 309, col: 613 },
        pixel: Pixel {
            red: 156,
            green: 152,
            blue: 148,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 309, col: 603 },
        pixel: Pixel {
            red: 244,
            green: 196,
            blue: 108,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 313, col: 591 },
        pixel: Pixel {
            red: 28,
            green: 24,
            blue: 8,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 297, col: 604 },
        pixel: Pixel {
            red: 88,
            green: 80,
            blue: 72,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 297, col: 592 },
        pixel: Pixel {
            red: 72,
            green: 64,
            blue: 32,
        },
    },
];

pub const WAYPOINT_MENU_VALIDATION_PIXELS: [ValidationPixel; 6] = [
    ValidationPixel {
        point: PointU16 { row: 242, col: 65 },
        pixel: Pixel {
            red: 68,
            green: 52,
            blue: 40,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 430, col: 390 },
        pixel: Pixel {
            red: 192,
            green: 160,
            blue: 128,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 14, col: 16 },
        pixel: Pixel {
            red: 172,
            green: 156,
            blue: 100,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 47, col: 386 },
        pixel: Pixel {
            red: 172,
            green: 156,
            blue: 100,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 489, col: 9 },
        pixel: Pixel {
            red: 172,
            green: 156,
            blue: 100,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 100, col: 250 },
        pixel: Pixel {
            red: 196,
            green: 196,
            blue: 196,
        },
    },
];

pub const IN_GAME_VALIDATION_PIXELS: [ValidationPixel; 6] = [
    ValidationPixel {
        point: PointU16 { row: 562, col: 785 },
        pixel: Pixel {
            red: 140,
            green: 124,
            blue: 80,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 553, col: 611 },
        pixel: Pixel {
            red: 116,
            green: 100,
            blue: 60,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 544, col: 18 },
        pixel: Pixel {
            red: 184,
            green: 184,
            blue: 184,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 592, col: 399 },
        pixel: Pixel {
            red: 224,
            green: 164,
            blue: 132,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 554, col: 207 },
        pixel: Pixel {
            red: 56,
            green: 40,
            blue: 28,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 554, col: 502 },
        pixel: Pixel {
            red: 72,
            green: 64,
            blue: 32,
        },
    },
];

pub const STASH_VALIDATION_PIXELS: [ValidationPixel; 6] = [
    ValidationPixel {
        point: PointU16 { row: 449, col: 356 },
        pixel: Pixel {
            red: 196,
            green: 196,
            blue: 196,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 449, col: 381 },
        pixel: Pixel {
            red: 196,
            green: 196,
            blue: 196,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 474, col: 356 },
        pixel: Pixel {
            red: 48,
            green: 48,
            blue: 48,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 57, col: 395 },
        pixel: Pixel {
            red: 96,
            green: 96,
            blue: 96,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 3, col: 342 },
        pixel: Pixel {
            red: 160,
            green: 160,
            blue: 160,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 3, col: 394 },
        pixel: Pixel {
            red: 160,
            green: 160,
            blue: 160,
        },
    },
];

pub const OPTIONS_MENU_VALIDATION_PIXELS: [ValidationPixel; 7] = [
    ValidationPixel {
        point: PointU16 { row: 256, col: 392 },
        pixel: Pixel {
            red: 116,
            green: 100,
            blue: 60,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 354, col: 529 },
        pixel: Pixel {
            red: 116,
            green: 100,
            blue: 60,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 353, col: 328 },
        pixel: Pixel {
            red: 140,
            green: 124,
            blue: 80,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 357, col: 250 },
        pixel: Pixel {
            red: 44,
            green: 44,
            blue: 44,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 305, col: 214 },
        pixel: Pixel {
            red: 140,
            green: 124,
            blue: 80,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 300, col: 311 },
        pixel: Pixel {
            red: 124,
            green: 116,
            blue: 112,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 210, col: 262 },
        pixel: Pixel {
            red: 116,
            green: 100,
            blue: 60,
        },
    },
];

pub const SINGLE_PLAYER_MENU_VALIDATION_PIXELS: [ValidationPixel; 6] = [
    ValidationPixel {
        point: PointU16 { row: 553, col: 748 },
        pixel: Pixel {
            red: 12,
            green: 12,
            blue: 40,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 553, col: 43 },
        pixel: Pixel {
            red: 44,
            green: 56,
            blue: 84,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 263, col: 35 },
        pixel: Pixel {
            red: 252,
            green: 252,
            blue: 196,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 15, col: 599 },
        pixel: Pixel {
            red: 252,
            green: 252,
            blue: 196,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 69, col: 375 },
        pixel: Pixel {
            red: 224,
            green: 196,
            blue: 148,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 15, col: 42 },
        pixel: Pixel {
            red: 224,
            green: 196,
            blue: 148,
        },
    },
];

pub const VIDEO_OPTIONS_MENU_VALIDATION_PIXELS: [ValidationPixel; 7] = [
    ValidationPixel {
        point: PointU16 { row: 92, col: 541 },
        pixel: Pixel {
            red: 116,
            green: 100,
            blue: 60,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 91, col: 444 },
        pixel: Pixel {
            red: 124,
            green: 116,
            blue: 112,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 89, col: 351 },
        pixel: Pixel {
            red: 140,
            green: 124,
            blue: 80,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 96, col: 261 },
        pixel: Pixel {
            red: 60,
            green: 52,
            blue: 24,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 190, col: 256 },
        pixel: Pixel {
            red: 140,
            green: 124,
            blue: 80,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 192, col: 423 },
        pixel: Pixel {
            red: 172,
            green: 156,
            blue: 100,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 368, col: 222 },
        pixel: Pixel {
            red: 140,
            green: 124,
            blue: 80,
        },
    },
];

pub const LIGHTING_QUALITY_LOW_VALIDATION_PIXELS: [ValidationPixel; 7] = [
    ValidationPixel {
        point: PointU16 { row: 184, col: 625 },
        pixel: Pixel {
            red: 84,
            green: 72,
            blue: 40,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 200, col: 616 },
        pixel: Pixel {
            red: 124,
            green: 116,
            blue: 112,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 184, col: 606 },
        pixel: Pixel {
            red: 40,
            green: 36,
            blue: 28,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 200, col: 591 },
        pixel: Pixel {
            red: 60,
            green: 52,
            blue: 24,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 184, col: 584 },
        pixel: Pixel {
            red: 28,
            green: 28,
            blue: 28,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 199, col: 574 },
        pixel: Pixel {
            red: 140,
            green: 124,
            blue: 80,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 184, col: 565 },
        pixel: Pixel {
            red: 100,
            green: 88,
            blue: 52,
        },
    },
];

pub const DIFFICULTY_MENU_VALIDATION_PIXELS: [ValidationPixel; 6] = [
    ValidationPixel {
        point: PointU16 { row: 201, col: 237 },
        pixel: Pixel {
            red: 72,
            green: 64,
            blue: 32,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 222, col: 505 },
        pixel: Pixel {
            red: 116,
            green: 100,
            blue: 60,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 227, col: 303 },
        pixel: Pixel {
            red: 100,
            green: 88,
            blue: 52,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 400, col: 562 },
        pixel: Pixel {
            red: 100,
            green: 88,
            blue: 52,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 400, col: 237 },
        pixel: Pixel {
            red: 84,
            green: 72,
            blue: 40,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 297, col: 403 },
        pixel: Pixel {
            red: 192,
            green: 160,
            blue: 128,
        },
    },
];

pub const HAS_DIED_VALIDATION_PIXELS: [ValidationPixel; 8] = [
    ValidationPixel {
        point: PointU16 { row: 190, col: 273 },
        pixel: Pixel {
            red: 176,
            green: 68,
            blue: 52,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 186, col: 298 },
        pixel: Pixel {
            red: 136,
            green: 48,
            blue: 36,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 193, col: 370 },
        pixel: Pixel {
            red: 128,
            green: 40,
            blue: 24,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 196, col: 472 },
        pixel: Pixel {
            red: 112,
            green: 36,
            blue: 24,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 196, col: 520 },
        pixel: Pixel {
            red: 112,
            green: 36,
            blue: 24,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 247, col: 607 },
        pixel: Pixel {
            red: 24,
            green: 8,
            blue: 4,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 244, col: 192 },
        pixel: Pixel {
            red: 44,
            green: 16,
            blue: 8,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 238, col: 261 },
        pixel: Pixel {
            red: 176,
            green: 68,
            blue: 52,
        },
    },
];

pub const AUTOMAP_OPTIONS_MENU_VALIDATION_PIXELS: [ValidationPixel; 7] = [
    ValidationPixel {
        point: PointU16 { row: 257, col: 501 },
        pixel: Pixel {
            red: 124,
            green: 116,
            blue: 112,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 298, col: 323 },
        pixel: Pixel {
            red: 124,
            green: 116,
            blue: 112,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 304, col: 235 },
        pixel: Pixel {
            red: 116,
            green: 100,
            blue: 60,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 301, col: 210 },
        pixel: Pixel {
            red: 140,
            green: 124,
            blue: 80,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 301, col: 174 },
        pixel: Pixel {
            red: 60,
            green: 52,
            blue: 24,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 253, col: 230 },
        pixel: Pixel {
            red: 172,
            green: 156,
            blue: 100,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 119, col: 242 },
        pixel: Pixel {
            red: 44,
            green: 36,
            blue: 16,
        },
    },
];

pub const EXIT_GAME_VALIDATION_PIXELS: [ValidationPixel; 6] = [
    ValidationPixel {
        point: PointU16 { row: 219, col: 504 },
        pixel: Pixel {
            red: 160,
            green: 120,
            blue: 64,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 220, col: 466 },
        pixel: Pixel {
            red: 244,
            green: 196,
            blue: 108,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 217, col: 378 },
        pixel: Pixel {
            red: 216,
            green: 184,
            blue: 100,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 226, col: 300 },
        pixel: Pixel {
            red: 204,
            green: 152,
            blue: 80,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 549, col: 529 },
        pixel: Pixel {
            red: 12,
            green: 12,
            blue: 40,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 547, col: 386 },
        pixel: Pixel {
            red: 4,
            green: 4,
            blue: 4,
        },
    },
];

pub const AUTOMAP_SIZE_FULL_VALIDATION_PIXELS: [ValidationPixel; 7] = [
    ValidationPixel {
        point: PointU16 { row: 176, col: 454 },
        pixel: Pixel {
            red: 244,
            green: 196,
            blue: 108,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 161, col: 462 },
        pixel: Pixel {
            red: 44,
            green: 44,
            blue: 44,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 173, col: 483 },
        pixel: Pixel {
            red: 84,
            green: 72,
            blue: 40,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 178, col: 499 },
        pixel: Pixel {
            red: 44,
            green: 36,
            blue: 16,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 163, col: 506 },
        pixel: Pixel {
            red: 28,
            green: 28,
            blue: 28,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 163, col: 607 },
        pixel: Pixel {
            red: 140,
            green: 124,
            blue: 80,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 178, col: 537 },
        pixel: Pixel {
            red: 116,
            green: 100,
            blue: 60,
        },
    },
];

pub const AUTOMAP_FADE_NO_VALIDATION_PIXELS: [ValidationPixel; 7] = [
    ValidationPixel {
        point: PointU16 { row: 221, col: 626 },
        pixel: Pixel {
            red: 28,
            green: 24,
            blue: 8,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 206, col: 617 },
        pixel: Pixel {
            red: 40,
            green: 36,
            blue: 28,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 219, col: 613 },
        pixel: Pixel {
            red: 156,
            green: 152,
            blue: 148,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 219, col: 603 },
        pixel: Pixel {
            red: 244,
            green: 196,
            blue: 108,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 223, col: 591 },
        pixel: Pixel {
            red: 28,
            green: 24,
            blue: 8,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 207, col: 604 },
        pixel: Pixel {
            red: 88,
            green: 80,
            blue: 72,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 207, col: 592 },
        pixel: Pixel {
            red: 72,
            green: 64,
            blue: 32,
        },
    },
];

pub const MENU_VALIDATION_PIXELS: [ValidationPixel; 11] = [
    ValidationPixel {
        point: PointU16 { row: 198, col: 331 },
        pixel: Pixel {
            red: 84,
            green: 72,
            blue: 40,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 209, col: 332 },
        pixel: Pixel {
            red: 132,
            green: 132,
            blue: 132,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 261, col: 242 },
        pixel: Pixel {
            red: 100,
            green: 88,
            blue: 52,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 259, col: 322 },
        pixel: Pixel {
            red: 60,
            green: 52,
            blue: 24,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 302, col: 542 },
        pixel: Pixel {
            red: 140,
            green: 124,
            blue: 80,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 300, col: 299 },
        pixel: Pixel {
            red: 116,
            green: 100,
            blue: 60,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 309, col: 337 },
        pixel: Pixel {
            red: 84,
            green: 72,
            blue: 40,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 307, col: 369 },
        pixel: Pixel {
            red: 140,
            green: 124,
            blue: 80,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 311, col: 410 },
        pixel: Pixel {
            red: 140,
            green: 124,
            blue: 80,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 306, col: 533 },
        pixel: Pixel {
            red: 72,
            green: 64,
            blue: 32,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 259, col: 192 },
        pixel: Pixel {
            red: 100,
            green: 88,
            blue: 52,
        },
    },
];

pub const INVENTORY_OPEN_VALIDATION_PIXELS: [ValidationPixel; 4] = [
    ValidationPixel {
        point: PointU16 { row: 451, col: 624 },
        pixel: Pixel {
            red: 52,
            green: 56,
            blue: 60,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 140, col: 616 },
        pixel: Pixel {
            red: 48,
            green: 48,
            blue: 48,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 91, col: 502 },
        pixel: Pixel {
            red: 76,
            green: 76,
            blue: 76,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 218, col: 501 },
        pixel: Pixel {
            red: 88,
            green: 80,
            blue: 72,
        },
    },
];

pub const BELT_OPEN_VALIDATION_PIXELS: [ValidationPixel; 4] = [
    ValidationPixel {
        point: PointU16 { row: 544, col: 545 },
        pixel: Pixel {
            red: 148,
            green: 128,
            blue: 100,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 530, col: 545 },
        pixel: Pixel {
            red: 252,
            green: 228,
            blue: 164,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 530, col: 421 },
        pixel: Pixel {
            red: 172,
            green: 156,
            blue: 100,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 553, col: 421 },
        pixel: Pixel {
            red: 172,
            green: 156,
            blue: 100,
        },
    },
];

pub const MERCHANT_TRADE_WINDOW_OPEN_VALIDATION_PIXELS: [ValidationPixel; 4] = [
    ValidationPixel {
        point: PointU16 { row: 473, col: 158 },
        pixel: Pixel {
            red: 24,
            green: 8,
            blue: 4,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 453, col: 365 },
        pixel: Pixel {
            red: 28,
            green: 20,
            blue: 16,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 221, col: 389 },
        pixel: Pixel {
            red: 92,
            green: 92,
            blue: 92,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 227, col: 85 },
        pixel: Pixel {
            red: 40,
            green: 36,
            blue: 28,
        },
    },
];

pub const START_SCREEN_LOADING_SCREEN_VALIDATION_PIXELS: [ValidationPixel; 6] = [
    ValidationPixel {
        point: PointU16 { row: 579, col: 0 },
        pixel: Pixel {
            red: 12,
            green: 12,
            blue: 12,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 579, col: 20 },
        pixel: Pixel {
            red: 4,
            green: 4,
            blue: 4,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 579, col: 244 },
        pixel: Pixel {
            red: 12,
            green: 12,
            blue: 12,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 579, col: 433 },
        pixel: Pixel {
            red: 20,
            green: 36,
            blue: 24,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 579, col: 667 },
        pixel: Pixel {
            red: 4,
            green: 4,
            blue: 4,
        },
    },
    ValidationPixel {
        point: PointU16 { row: 579, col: 780 },
        pixel: Pixel {
            red: 4,
            green: 4,
            blue: 4,
        },
    },
];
