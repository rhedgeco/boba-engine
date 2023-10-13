pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Color {
    pub const fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }
}

impl Color {
    pub const ALICE_BLUE: Self = Self::new(240. / 255., 248. / 255., 255. / 255., 1.);
    pub const ANTIQUE_WHITE: Self = Self::new(250. / 255., 235. / 255., 215. / 255., 1.);
    pub const AQUA: Self = Self::new(0. / 255., 255. / 255., 255. / 255., 1.);
    pub const AQUAMARINE: Self = Self::new(127. / 255., 255. / 255., 212. / 255., 1.);
    pub const AZURE: Self = Self::new(240. / 255., 255. / 255., 255. / 255., 1.);
    pub const BEIGE: Self = Self::new(245. / 255., 245. / 255., 220. / 255., 1.);
    pub const BISQUE: Self = Self::new(255. / 255., 228. / 255., 196. / 255., 1.);
    pub const BLACK: Self = Self::new(0. / 255., 0. / 255., 0. / 255., 1.);
    pub const BLANCHED_ALMOND: Self = Self::new(255. / 255., 235. / 255., 205. / 255., 1.);
    pub const BLUE: Self = Self::new(0. / 255., 0. / 255., 255. / 255., 1.);
    pub const BLUE_VIOLET: Self = Self::new(138. / 255., 43. / 255., 226. / 255., 1.);
    pub const BROWN: Self = Self::new(165. / 255., 42. / 255., 42. / 255., 1.);
    pub const BURLY_WOOD: Self = Self::new(222. / 255., 184. / 255., 135. / 255., 1.);
    pub const CADET_BLUE: Self = Self::new(95. / 255., 158. / 255., 160. / 255., 1.);
    pub const CHARTREUSE: Self = Self::new(127. / 255., 255. / 255., 0. / 255., 1.);
    pub const CHOCOLATE: Self = Self::new(210. / 255., 105. / 255., 30. / 255., 1.);
    pub const CORAL: Self = Self::new(255. / 255., 127. / 255., 80. / 255., 1.);
    pub const CORNFLOWER_BLUE: Self = Self::new(100. / 255., 149. / 255., 237. / 255., 1.);
    pub const CORNSILK: Self = Self::new(255. / 255., 248. / 255., 220. / 255., 1.);
    pub const CRIMSON: Self = Self::new(220. / 255., 20. / 255., 60. / 255., 1.);
    pub const CYAN: Self = Self::new(0. / 255., 255. / 255., 255. / 255., 1.);
    pub const DARK_BLUE: Self = Self::new(0. / 255., 0. / 255., 139. / 255., 1.);
    pub const DARK_CYAN: Self = Self::new(0. / 255., 139. / 255., 139. / 255., 1.);
    pub const DARK_GOLDEN_ROD: Self = Self::new(184. / 255., 134. / 255., 11. / 255., 1.);
    pub const DARK_GREY: Self = Self::new(169. / 255., 169. / 255., 169. / 255., 1.);
    pub const DARK_GREEN: Self = Self::new(0. / 255., 100. / 255., 0. / 255., 1.);
    pub const DARK_KHAKI: Self = Self::new(189. / 255., 183. / 255., 107. / 255., 1.);
    pub const DARK_MAGENTA: Self = Self::new(139. / 255., 0. / 255., 139. / 255., 1.);
    pub const DARK_OLIVE_GREEN: Self = Self::new(85. / 255., 107. / 255., 47. / 255., 1.);
    pub const DARKORANGE: Self = Self::new(255. / 255., 140. / 255., 0. / 255., 1.);
    pub const DARK_ORCHID: Self = Self::new(153. / 255., 50. / 255., 204. / 255., 1.);
    pub const DARK_RED: Self = Self::new(139. / 255., 0. / 255., 0. / 255., 1.);
    pub const DARK_SALMON: Self = Self::new(233. / 255., 150. / 255., 122. / 255., 1.);
    pub const DARK_SEA_GREEN: Self = Self::new(143. / 255., 188. / 255., 143. / 255., 1.);
    pub const DARK_SLATE_BLUE: Self = Self::new(72. / 255., 61. / 255., 139. / 255., 1.);
    pub const DARK_SLATE_GREY: Self = Self::new(47. / 255., 79. / 255., 79. / 255., 1.);
    pub const DARK_TURQUOISE: Self = Self::new(0. / 255., 206. / 255., 209. / 255., 1.);
    pub const DARK_VIOLET: Self = Self::new(148. / 255., 0. / 255., 211. / 255., 1.);
    pub const DEEP_PINK: Self = Self::new(255. / 255., 20. / 255., 147. / 255., 1.);
    pub const DEEP_SKY_BLUE: Self = Self::new(0. / 255., 191. / 255., 255. / 255., 1.);
    pub const DIM_GRAY: Self = Self::new(105. / 255., 105. / 255., 105. / 255., 1.);
    pub const DODGER_BLUE: Self = Self::new(30. / 255., 144. / 255., 255. / 255., 1.);
    pub const FIRE_BRICK: Self = Self::new(178. / 255., 34. / 255., 34. / 255., 1.);
    pub const FLORAL_WHITE: Self = Self::new(255. / 255., 250. / 255., 240. / 255., 1.);
    pub const FOREST_GREEN: Self = Self::new(34. / 255., 139. / 255., 34. / 255., 1.);
    pub const FUCHSIA: Self = Self::new(255. / 255., 0. / 255., 255. / 255., 1.);
    pub const GAINSBORO: Self = Self::new(220. / 255., 220. / 255., 220. / 255., 1.);
    pub const GHOST_WHITE: Self = Self::new(248. / 255., 248. / 255., 255. / 255., 1.);
    pub const GOLD: Self = Self::new(255. / 255., 215. / 255., 0. / 255., 1.);
    pub const GOLDEN_ROD: Self = Self::new(218. / 255., 165. / 255., 32. / 255., 1.);
    pub const GREY: Self = Self::new(128. / 255., 128. / 255., 128. / 255., 1.);
    pub const GREEN: Self = Self::new(0. / 255., 128. / 255., 0. / 255., 1.);
    pub const GREEN_YELLOW: Self = Self::new(173. / 255., 255. / 255., 47. / 255., 1.);
    pub const HONEY_DEW: Self = Self::new(240. / 255., 255. / 255., 240. / 255., 1.);
    pub const HOT_PINK: Self = Self::new(255. / 255., 105. / 255., 180. / 255., 1.);
    pub const INDIAN_RED: Self = Self::new(205. / 255., 92. / 255., 92. / 255., 1.);
    pub const INDIGO: Self = Self::new(75. / 255., 0. / 255., 130. / 255., 1.);
    pub const IVORY: Self = Self::new(255. / 255., 255. / 255., 240. / 255., 1.);
    pub const KHAKI: Self = Self::new(240. / 255., 230. / 255., 140. / 255., 1.);
    pub const LAVENDER: Self = Self::new(230. / 255., 230. / 255., 250. / 255., 1.);
    pub const LAVENDER_BLUSH: Self = Self::new(255. / 255., 240. / 255., 245. / 255., 1.);
    pub const LAWN_GREEN: Self = Self::new(124. / 255., 252. / 255., 0. / 255., 1.);
    pub const LEMON_CHIFFON: Self = Self::new(255. / 255., 250. / 255., 205. / 255., 1.);
    pub const LIGHT_BLUE: Self = Self::new(173. / 255., 216. / 255., 230. / 255., 1.);
    pub const LIGHT_CORAL: Self = Self::new(240. / 255., 128. / 255., 128. / 255., 1.);
    pub const LIGHT_CYAN: Self = Self::new(224. / 255., 255. / 255., 255. / 255., 1.);
    pub const LIGHT_GOLDEN_ROD_YELLOW: Self = Self::new(250. / 255., 250. / 255., 210. / 255., 1.);
    pub const LIGHT_GREY: Self = Self::new(211. / 255., 211. / 255., 211. / 255., 1.);
    pub const LIGHT_GREEN: Self = Self::new(144. / 255., 238. / 255., 144. / 255., 1.);
    pub const LIGHT_PINK: Self = Self::new(255. / 255., 182. / 255., 193. / 255., 1.);
    pub const LIGHT_SALMON: Self = Self::new(255. / 255., 160. / 255., 122. / 255., 1.);
    pub const LIGHT_SEA_GREEN: Self = Self::new(32. / 255., 178. / 255., 170. / 255., 1.);
    pub const LIGHT_SKY_BLUE: Self = Self::new(135. / 255., 206. / 255., 250. / 255., 1.);
    pub const LIGHT_SLATE_GREY: Self = Self::new(119. / 255., 136. / 255., 153. / 255., 1.);
    pub const LIGHT_STEEL_BLUE: Self = Self::new(176. / 255., 196. / 255., 222. / 255., 1.);
    pub const LIGHT_YELLOW: Self = Self::new(255. / 255., 255. / 255., 224. / 255., 1.);
    pub const LIME: Self = Self::new(0. / 255., 255. / 255., 0. / 255., 1.);
    pub const LIME_GREEN: Self = Self::new(50. / 255., 205. / 255., 50. / 255., 1.);
    pub const LINEN: Self = Self::new(250. / 255., 240. / 255., 230. / 255., 1.);
    pub const MAGENTA: Self = Self::new(255. / 255., 0. / 255., 255. / 255., 1.);
    pub const MAROON: Self = Self::new(128. / 255., 0. / 255., 0. / 255., 1.);
    pub const MEDIUM_AQUA_MARINE: Self = Self::new(102. / 255., 205. / 255., 170. / 255., 1.);
    pub const MEDIUM_BLUE: Self = Self::new(0. / 255., 0. / 255., 205. / 255., 1.);
    pub const MEDIUM_ORCHID: Self = Self::new(186. / 255., 85. / 255., 211. / 255., 1.);
    pub const MEDIUM_PURPLE: Self = Self::new(147. / 255., 112. / 255., 216. / 255., 1.);
    pub const MEDIUM_SEA_GREEN: Self = Self::new(60. / 255., 179. / 255., 113. / 255., 1.);
    pub const MEDIUM_SLATE_BLUE: Self = Self::new(123. / 255., 104. / 255., 238. / 255., 1.);
    pub const MEDIUM_SPRING_GREEN: Self = Self::new(0. / 255., 250. / 255., 154. / 255., 1.);
    pub const MEDIUM_TURQUOISE: Self = Self::new(72. / 255., 209. / 255., 204. / 255., 1.);
    pub const MEDIUM_VIOLET_RED: Self = Self::new(199. / 255., 21. / 255., 133. / 255., 1.);
    pub const MIDNIGHT_BLUE: Self = Self::new(25. / 255., 25. / 255., 112. / 255., 1.);
    pub const MINT_CREAM: Self = Self::new(245. / 255., 255. / 255., 250. / 255., 1.);
    pub const MISTY_ROSE: Self = Self::new(255. / 255., 228. / 255., 225. / 255., 1.);
    pub const MOCCASIN: Self = Self::new(255. / 255., 228. / 255., 181. / 255., 1.);
    pub const NAVAJO_WHITE: Self = Self::new(255. / 255., 222. / 255., 173. / 255., 1.);
    pub const NAVY: Self = Self::new(0. / 255., 0. / 255., 128. / 255., 1.);
    pub const OLD_LACE: Self = Self::new(253. / 255., 245. / 255., 230. / 255., 1.);
    pub const OLIVE: Self = Self::new(128. / 255., 128. / 255., 0. / 255., 1.);
    pub const OLIVE_DRAB: Self = Self::new(107. / 255., 142. / 255., 35. / 255., 1.);
    pub const ORANGE: Self = Self::new(255. / 255., 165. / 255., 0. / 255., 1.);
    pub const ORANGE_RED: Self = Self::new(255. / 255., 69. / 255., 0. / 255., 1.);
    pub const ORCHID: Self = Self::new(218. / 255., 112. / 255., 214. / 255., 1.);
    pub const PALE_GOLDEN_ROD: Self = Self::new(238. / 255., 232. / 255., 170. / 255., 1.);
    pub const PALE_GREEN: Self = Self::new(152. / 255., 251. / 255., 152. / 255., 1.);
    pub const PALE_TURQUOISE: Self = Self::new(175. / 255., 238. / 255., 238. / 255., 1.);
    pub const PALE_VIOLET_RED: Self = Self::new(216. / 255., 112. / 255., 147. / 255., 1.);
    pub const PAPAYA_WHIP: Self = Self::new(255. / 255., 239. / 255., 213. / 255., 1.);
    pub const PEACH_PUFF: Self = Self::new(255. / 255., 218. / 255., 185. / 255., 1.);
    pub const PERU: Self = Self::new(205. / 255., 133. / 255., 63. / 255., 1.);
    pub const PINK: Self = Self::new(255. / 255., 192. / 255., 203. / 255., 1.);
    pub const PLUM: Self = Self::new(221. / 255., 160. / 255., 221. / 255., 1.);
    pub const POWDER_BLUE: Self = Self::new(176. / 255., 224. / 255., 230. / 255., 1.);
    pub const PURPLE: Self = Self::new(128. / 255., 0. / 255., 128. / 255., 1.);
    pub const RED: Self = Self::new(255. / 255., 0. / 255., 0. / 255., 1.);
    pub const ROSY_BROWN: Self = Self::new(188. / 255., 143. / 255., 143. / 255., 1.);
    pub const ROYAL_BLUE: Self = Self::new(65. / 255., 105. / 255., 225. / 255., 1.);
    pub const SADDLE_BROWN: Self = Self::new(139. / 255., 69. / 255., 19. / 255., 1.);
    pub const SALMON: Self = Self::new(250. / 255., 128. / 255., 114. / 255., 1.);
    pub const SANDY_BROWN: Self = Self::new(244. / 255., 164. / 255., 96. / 255., 1.);
    pub const SEA_GREEN: Self = Self::new(46. / 255., 139. / 255., 87. / 255., 1.);
    pub const SEA_SHELL: Self = Self::new(255. / 255., 245. / 255., 238. / 255., 1.);
    pub const SIENNA: Self = Self::new(160. / 255., 82. / 255., 45. / 255., 1.);
    pub const SILVER: Self = Self::new(192. / 255., 192. / 255., 192. / 255., 1.);
    pub const SKY_BLUE: Self = Self::new(135. / 255., 206. / 255., 235. / 255., 1.);
    pub const SLATE_BLUE: Self = Self::new(106. / 255., 90. / 255., 205. / 255., 1.);
    pub const SLATE_GREY: Self = Self::new(112. / 255., 128. / 255., 144. / 255., 1.);
    pub const SNOW: Self = Self::new(255. / 255., 250. / 255., 250. / 255., 1.);
    pub const SPRING_GREEN: Self = Self::new(0. / 255., 255. / 255., 127. / 255., 1.);
    pub const STEEL_BLUE: Self = Self::new(70. / 255., 130. / 255., 180. / 255., 1.);
    pub const TAN: Self = Self::new(210. / 255., 180. / 255., 140. / 255., 1.);
    pub const TEAL: Self = Self::new(0. / 255., 128. / 255., 128. / 255., 1.);
    pub const THISTLE: Self = Self::new(216. / 255., 191. / 255., 216. / 255., 1.);
    pub const TOMATO: Self = Self::new(255. / 255., 99. / 255., 71. / 255., 1.);
    pub const TURQUOISE: Self = Self::new(64. / 255., 224. / 255., 208. / 255., 1.);
    pub const VIOLET: Self = Self::new(238. / 255., 130. / 255., 238. / 255., 1.);
    pub const WHEAT: Self = Self::new(245. / 255., 222. / 255., 179. / 255., 1.);
    pub const WHITE: Self = Self::new(255. / 255., 255. / 255., 255. / 255., 1.);
    pub const WHITE_SMOKE: Self = Self::new(245. / 255., 245. / 255., 245. / 255., 1.);
    pub const YELLOW: Self = Self::new(255. / 255., 255. / 255., 0. / 255., 1.);
    pub const YELLOW_GREEN: Self = Self::new(154. / 255., 205. / 255., 50. / 255., 1.);
}
