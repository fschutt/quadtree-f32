extern crate quadtree_f32;

use quadtree_f32::{Item, ItemId, QuadTree, Rect};
use std::time::Instant;

fn main() {
    let items = vec![
        (
            ItemId(0),
            Item::Rect(Rect {
                max_x: 17.470589,
                max_y: 0.0,
                min_x: 17.470589,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1),
            Item::Rect(Rect {
                max_x: 17.470589,
                max_y: 0.0,
                min_x: 17.470589,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(2),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 9.130435,
                min_x: 0.0,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(3),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 9.130435,
                min_x: 5.8235292,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(4),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 9.130435,
                min_x: 11.6470585,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(5),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 9.130435,
                min_x: 11.6470585,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(6),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 9.130435,
                min_x: 5.8235292,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(7),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 9.130435,
                min_x: 0.0,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(8),
            Item::Rect(Rect {
                max_x: 104.823524,
                max_y: 127.82609,
                min_x: 104.823524,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(9),
            Item::Rect(Rect {
                max_x: 104.823524,
                max_y: 127.82609,
                min_x: 104.823524,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(10),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 127.82609,
                min_x: 104.823524,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(11),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 118.695656,
                min_x: 110.64706,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(12),
            Item::Rect(Rect {
                max_x: 116.47058,
                max_y: 127.82609,
                min_x: 110.64706,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(13),
            Item::Rect(Rect {
                max_x: 116.47058,
                max_y: 127.82609,
                min_x: 116.47058,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(14),
            Item::Rect(Rect {
                max_x: 116.47058,
                max_y: 127.82609,
                min_x: 116.47058,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(15),
            Item::Rect(Rect {
                max_x: 116.47058,
                max_y: 127.82609,
                min_x: 110.64706,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(16),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 127.82609,
                min_x: 104.823524,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(17),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 36.52174,
                min_x: 250.41176,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(18),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 36.52174,
                min_x: 250.41176,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(19),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 36.52174,
                min_x: 250.41176,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(20),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 36.52174,
                min_x: 250.41176,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(21),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 54.782608,
                min_x: 262.0588,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(22),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 54.782608,
                min_x: 262.0588,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(23),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 54.782608,
                min_x: 262.0588,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(24),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 54.782608,
                min_x: 262.0588,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(25),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 9.130435,
                min_x: 267.88235,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(26),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 9.130435,
                min_x: 267.88235,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(27),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 9.130435,
                min_x: 267.88235,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(28),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 9.130435,
                min_x: 267.88235,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(29),
            Item::Rect(Rect {
                max_x: 145.58823,
                max_y: 182.6087,
                min_x: 145.58823,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(30),
            Item::Rect(Rect {
                max_x: 145.58823,
                max_y: 182.6087,
                min_x: 145.58823,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(31),
            Item::Rect(Rect {
                max_x: 145.58823,
                max_y: 182.6087,
                min_x: 145.58823,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(32),
            Item::Rect(Rect {
                max_x: 145.58823,
                max_y: 182.6087,
                min_x: 145.58823,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(33),
            Item::Rect(Rect {
                max_x: 23.294117,
                max_y: 9.130435,
                min_x: 23.294117,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(34),
            Item::Rect(Rect {
                max_x: 23.294117,
                max_y: 9.130435,
                min_x: 23.294117,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(35),
            Item::Rect(Rect {
                max_x: 29.117645,
                max_y: 9.130435,
                min_x: 23.294117,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(36),
            Item::Rect(Rect {
                max_x: 29.117645,
                max_y: 9.130435,
                min_x: 29.117645,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(37),
            Item::Rect(Rect {
                max_x: 29.117645,
                max_y: 9.130435,
                min_x: 29.117645,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(38),
            Item::Rect(Rect {
                max_x: 29.117645,
                max_y: 9.130435,
                min_x: 23.294117,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(39),
            Item::Rect(Rect {
                max_x: 52.411762,
                max_y: 82.17391,
                min_x: 52.411762,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(40),
            Item::Rect(Rect {
                max_x: 52.411762,
                max_y: 82.17391,
                min_x: 52.411762,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(41),
            Item::Rect(Rect {
                max_x: 52.411762,
                max_y: 82.17391,
                min_x: 52.411762,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(42),
            Item::Rect(Rect {
                max_x: 52.411762,
                max_y: 82.17391,
                min_x: 52.411762,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(43),
            Item::Rect(Rect {
                max_x: 104.823524,
                max_y: 109.565216,
                min_x: 104.823524,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(44),
            Item::Rect(Rect {
                max_x: 104.823524,
                max_y: 109.565216,
                min_x: 104.823524,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(45),
            Item::Rect(Rect {
                max_x: 104.823524,
                max_y: 109.565216,
                min_x: 104.823524,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(46),
            Item::Rect(Rect {
                max_x: 104.823524,
                max_y: 109.565216,
                min_x: 104.823524,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(47),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 118.695656,
                min_x: 110.64706,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(48),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 82.17391,
                min_x: 279.52942,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(49),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 82.17391,
                min_x: 279.52942,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(50),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 82.17391,
                min_x: 279.52942,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(51),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 82.17391,
                min_x: 279.52942,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(52),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 0.33816427,
                min_x: 256.01132,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(53),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 0.0,
                min_x: 262.0588,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(54),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 0.33816427,
                min_x: 256.2353,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(55),
            Item::Rect(Rect {
                max_x: 40.764706,
                max_y: 9.130435,
                min_x: 34.941177,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(56),
            Item::Rect(Rect {
                max_x: 40.764706,
                max_y: 9.130435,
                min_x: 40.764706,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(57),
            Item::Rect(Rect {
                max_x: 43.676468,
                max_y: 18.26087,
                min_x: 40.764706,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(58),
            Item::Rect(Rect {
                max_x: 43.676468,
                max_y: 22.826088,
                min_x: 40.764706,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(59),
            Item::Rect(Rect {
                max_x: 40.764706,
                max_y: 22.826088,
                min_x: 34.941177,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(60),
            Item::Rect(Rect {
                max_x: 34.941177,
                max_y: 18.26087,
                min_x: 29.117645,
                min_y: 15.217392,
            }),
        ),
        (
            ItemId(61),
            Item::Rect(Rect {
                max_x: 29.117645,
                max_y: 15.217392,
                min_x: 23.294117,
                min_y: 15.217392,
            }),
        ),
        (
            ItemId(62),
            Item::Rect(Rect {
                max_x: 23.294117,
                max_y: 15.217392,
                min_x: 17.470589,
                min_y: 13.695652,
            }),
        ),
        (
            ItemId(63),
            Item::Rect(Rect {
                max_x: 17.470589,
                max_y: 15.217392,
                min_x: 11.6470585,
                min_y: 13.695652,
            }),
        ),
        (
            ItemId(64),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 15.217392,
                min_x: 5.8235292,
                min_y: 13.695652,
            }),
        ),
        (
            ItemId(65),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 15.217392,
                min_x: 0.0,
                min_y: 13.695652,
            }),
        ),
        (
            ItemId(66),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 36.52174,
                min_x: 2.9117646,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(67),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 36.52174,
                min_x: 5.8235292,
                min_y: 33.478264,
            }),
        ),
        (
            ItemId(68),
            Item::Rect(Rect {
                max_x: 17.470589,
                max_y: 33.478264,
                min_x: 11.6470585,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(69),
            Item::Rect(Rect {
                max_x: 17.470589,
                max_y: 27.391304,
                min_x: 17.470589,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(70),
            Item::Rect(Rect {
                max_x: 17.470589,
                max_y: 36.52174,
                min_x: 17.470589,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(71),
            Item::Rect(Rect {
                max_x: 17.470589,
                max_y: 45.652176,
                min_x: 17.470589,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(72),
            Item::Rect(Rect {
                max_x: 23.294117,
                max_y: 54.782608,
                min_x: 17.470589,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(73),
            Item::Rect(Rect {
                max_x: 23.294117,
                max_y: 54.782608,
                min_x: 23.294117,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(74),
            Item::Rect(Rect {
                max_x: 23.294117,
                max_y: 54.782608,
                min_x: 23.294117,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(75),
            Item::Rect(Rect {
                max_x: 23.294117,
                max_y: 54.782608,
                min_x: 17.470589,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(76),
            Item::Rect(Rect {
                max_x: 17.470589,
                max_y: 54.782608,
                min_x: 11.6470585,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(77),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 45.652176,
                min_x: 5.8235292,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(78),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 36.52174,
                min_x: 0.0,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(79),
            Item::Rect(Rect {
                max_x: 87.352936,
                max_y: 0.0,
                min_x: 87.352936,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(80),
            Item::Rect(Rect {
                max_x: 87.352936,
                max_y: 4.5652175,
                min_x: 81.52941,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(81),
            Item::Rect(Rect {
                max_x: 81.52941,
                max_y: 9.130435,
                min_x: 78.617645,
                min_y: 4.5652175,
            }),
        ),
        (
            ItemId(82),
            Item::Rect(Rect {
                max_x: 78.617645,
                max_y: 13.695652,
                min_x: 75.70588,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(83),
            Item::Rect(Rect {
                max_x: 75.70588,
                max_y: 13.695652,
                min_x: 69.882355,
                min_y: 13.695652,
            }),
        ),
        (
            ItemId(84),
            Item::Rect(Rect {
                max_x: 69.882355,
                max_y: 18.26087,
                min_x: 64.05882,
                min_y: 13.695652,
            }),
        ),
        (
            ItemId(85),
            Item::Rect(Rect {
                max_x: 69.882355,
                max_y: 27.391304,
                min_x: 64.05882,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(86),
            Item::Rect(Rect {
                max_x: 75.70588,
                max_y: 27.391304,
                min_x: 69.882355,
                min_y: 22.826088,
            }),
        ),
        (
            ItemId(87),
            Item::Rect(Rect {
                max_x: 81.52941,
                max_y: 27.391304,
                min_x: 75.70588,
                min_y: 22.826088,
            }),
        ),
        (
            ItemId(88),
            Item::Rect(Rect {
                max_x: 87.352936,
                max_y: 27.391304,
                min_x: 81.52941,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(89),
            Item::Rect(Rect {
                max_x: 87.352936,
                max_y: 18.26087,
                min_x: 87.352936,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(90),
            Item::Rect(Rect {
                max_x: 87.352936,
                max_y: 27.391304,
                min_x: 87.352936,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(91),
            Item::Rect(Rect {
                max_x: 87.352936,
                max_y: 36.52174,
                min_x: 81.52941,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(92),
            Item::Rect(Rect {
                max_x: 87.352936,
                max_y: 45.652176,
                min_x: 81.52941,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(93),
            Item::Rect(Rect {
                max_x: 87.352936,
                max_y: 45.652176,
                min_x: 87.352936,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(94),
            Item::Rect(Rect {
                max_x: 87.352936,
                max_y: 45.652176,
                min_x: 87.352936,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(95),
            Item::Rect(Rect {
                max_x: 87.352936,
                max_y: 45.652176,
                min_x: 81.52941,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(96),
            Item::Rect(Rect {
                max_x: 81.52941,
                max_y: 45.652176,
                min_x: 75.70588,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(97),
            Item::Rect(Rect {
                max_x: 75.70588,
                max_y: 54.782608,
                min_x: 72.79411,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(98),
            Item::Rect(Rect {
                max_x: 72.79411,
                max_y: 59.347828,
                min_x: 69.882355,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(99),
            Item::Rect(Rect {
                max_x: 69.882355,
                max_y: 63.913044,
                min_x: 64.05882,
                min_y: 59.347828,
            }),
        ),
        (
            ItemId(100),
            Item::Rect(Rect {
                max_x: 64.05882,
                max_y: 73.04348,
                min_x: 58.23529,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(101),
            Item::Rect(Rect {
                max_x: 64.05882,
                max_y: 82.17391,
                min_x: 58.23529,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(102),
            Item::Rect(Rect {
                max_x: 64.05882,
                max_y: 82.17391,
                min_x: 64.05882,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(103),
            Item::Rect(Rect {
                max_x: 69.882355,
                max_y: 91.30435,
                min_x: 64.05882,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(104),
            Item::Rect(Rect {
                max_x: 75.70588,
                max_y: 91.30435,
                min_x: 69.882355,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(105),
            Item::Rect(Rect {
                max_x: 75.70588,
                max_y: 91.30435,
                min_x: 75.70588,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(106),
            Item::Rect(Rect {
                max_x: 75.70588,
                max_y: 91.30435,
                min_x: 75.70588,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(107),
            Item::Rect(Rect {
                max_x: 75.70588,
                max_y: 91.30435,
                min_x: 69.882355,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(108),
            Item::Rect(Rect {
                max_x: 69.882355,
                max_y: 91.30435,
                min_x: 64.05882,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(109),
            Item::Rect(Rect {
                max_x: 64.05882,
                max_y: 91.30435,
                min_x: 58.23529,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(110),
            Item::Rect(Rect {
                max_x: 58.23529,
                max_y: 91.30435,
                min_x: 52.411762,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(111),
            Item::Rect(Rect {
                max_x: 52.411762,
                max_y: 100.434784,
                min_x: 46.588234,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(112),
            Item::Rect(Rect {
                max_x: 46.588234,
                max_y: 100.434784,
                min_x: 46.588234,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(113),
            Item::Rect(Rect {
                max_x: 46.588234,
                max_y: 91.30435,
                min_x: 40.764706,
                min_y: 85.21739,
            }),
        ),
        (
            ItemId(114),
            Item::Rect(Rect {
                max_x: 40.764706,
                max_y: 85.21739,
                min_x: 38.82353,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(115),
            Item::Rect(Rect {
                max_x: 40.764706,
                max_y: 82.17391,
                min_x: 38.82353,
                min_y: 79.13044,
            }),
        ),
        (
            ItemId(116),
            Item::Rect(Rect {
                max_x: 46.588234,
                max_y: 79.13044,
                min_x: 40.764706,
                min_y: 79.13044,
            }),
        ),
        (
            ItemId(117),
            Item::Rect(Rect {
                max_x: 52.411762,
                max_y: 79.13044,
                min_x: 46.588234,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(118),
            Item::Rect(Rect {
                max_x: 52.411762,
                max_y: 73.04348,
                min_x: 52.411762,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(119),
            Item::Rect(Rect {
                max_x: 52.411762,
                max_y: 63.913044,
                min_x: 46.588234,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(120),
            Item::Rect(Rect {
                max_x: 52.411762,
                max_y: 54.782608,
                min_x: 46.588234,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(121),
            Item::Rect(Rect {
                max_x: 52.411762,
                max_y: 54.782608,
                min_x: 52.411762,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(122),
            Item::Rect(Rect {
                max_x: 58.23529,
                max_y: 63.913044,
                min_x: 52.411762,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(123),
            Item::Rect(Rect {
                max_x: 64.05882,
                max_y: 63.913044,
                min_x: 58.23529,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(124),
            Item::Rect(Rect {
                max_x: 64.05882,
                max_y: 54.782608,
                min_x: 64.05882,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(125),
            Item::Rect(Rect {
                max_x: 69.882355,
                max_y: 54.782608,
                min_x: 64.05882,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(126),
            Item::Rect(Rect {
                max_x: 69.882355,
                max_y: 45.652176,
                min_x: 69.882355,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(127),
            Item::Rect(Rect {
                max_x: 72.79411,
                max_y: 45.652176,
                min_x: 69.882355,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(128),
            Item::Rect(Rect {
                max_x: 72.79411,
                max_y: 36.52174,
                min_x: 69.882355,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(129),
            Item::Rect(Rect {
                max_x: 69.882355,
                max_y: 27.391304,
                min_x: 64.05882,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(130),
            Item::Rect(Rect {
                max_x: 64.05882,
                max_y: 27.391304,
                min_x: 58.23529,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(131),
            Item::Rect(Rect {
                max_x: 58.23529,
                max_y: 27.391304,
                min_x: 52.411762,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(132),
            Item::Rect(Rect {
                max_x: 52.411762,
                max_y: 27.391304,
                min_x: 46.588234,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(133),
            Item::Rect(Rect {
                max_x: 46.588234,
                max_y: 27.391304,
                min_x: 46.588234,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(134),
            Item::Rect(Rect {
                max_x: 46.588234,
                max_y: 27.391304,
                min_x: 46.588234,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(135),
            Item::Rect(Rect {
                max_x: 52.411762,
                max_y: 27.391304,
                min_x: 46.588234,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(136),
            Item::Rect(Rect {
                max_x: 52.411762,
                max_y: 18.26087,
                min_x: 52.411762,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(137),
            Item::Rect(Rect {
                max_x: 52.411762,
                max_y: 9.130435,
                min_x: 52.411762,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(138),
            Item::Rect(Rect {
                max_x: 58.23529,
                max_y: 9.130435,
                min_x: 52.411762,
                min_y: 6.086957,
            }),
        ),
        (
            ItemId(139),
            Item::Rect(Rect {
                max_x: 64.05882,
                max_y: 6.086957,
                min_x: 58.23529,
                min_y: 4.5652175,
            }),
        ),
        (
            ItemId(140),
            Item::Rect(Rect {
                max_x: 69.882355,
                max_y: 4.5652175,
                min_x: 64.05882,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(141),
            Item::Rect(Rect {
                max_x: 2.9117646,
                max_y: 27.391304,
                min_x: 0.0,
                min_y: 22.826088,
            }),
        ),
        (
            ItemId(142),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 8.521739,
                min_x: 5.8235292,
                min_y: 8.478261,
            }),
        ),
        (
            ItemId(143),
            Item::Rect(Rect {
                max_x: 17.082354,
                max_y: 8.521739,
                min_x: 11.6470585,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(144),
            Item::Rect(Rect {
                max_x: 275.64703,
                max_y: 9.130435,
                min_x: 273.70587,
                min_y: 6.086957,
            }),
        ),
        (
            ItemId(145),
            Item::Rect(Rect {
                max_x: 275.64703,
                max_y: 13.695652,
                min_x: 273.70587,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(146),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 13.695652,
                min_x: 267.88235,
                min_y: 9.739131,
            }),
        ),
        (
            ItemId(147),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 9.739131,
                min_x: 267.50662,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(148),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 9.130435,
                min_x: 267.50662,
                min_y: 4.5652175,
            }),
        ),
        (
            ItemId(149),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 6.086957,
                min_x: 267.88235,
                min_y: 4.5652175,
            }),
        ),
        (
            ItemId(150),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 8.541375,
                min_x: 0.0,
                min_y: 8.478261,
            }),
        ),
        (
            ItemId(151),
            Item::Rect(Rect {
                max_x: 209.64705,
                max_y: 0.0,
                min_x: 209.64705,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(152),
            Item::Rect(Rect {
                max_x: 209.64705,
                max_y: 0.0,
                min_x: 209.64705,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(153),
            Item::Rect(Rect {
                max_x: 262.49017,
                max_y: 54.782608,
                min_x: 262.0588,
                min_y: 54.10628,
            }),
        ),
        (
            ItemId(154),
            Item::Rect(Rect {
                max_x: 262.49017,
                max_y: 57.826088,
                min_x: 262.0588,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(155),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 57.826088,
                min_x: 258.17645,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(156),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 54.782608,
                min_x: 258.17645,
                min_y: 54.10628,
            }),
        ),
        (
            ItemId(157),
            Item::Rect(Rect {
                max_x: 64.05882,
                max_y: 63.913044,
                min_x: 64.05882,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(158),
            Item::Rect(Rect {
                max_x: 34.941177,
                max_y: 54.782608,
                min_x: 34.941177,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(159),
            Item::Rect(Rect {
                max_x: 34.941177,
                max_y: 54.782608,
                min_x: 29.117645,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(160),
            Item::Rect(Rect {
                max_x: 29.117645,
                max_y: 45.652176,
                min_x: 29.117645,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(161),
            Item::Rect(Rect {
                max_x: 29.117645,
                max_y: 45.652176,
                min_x: 29.117645,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(162),
            Item::Rect(Rect {
                max_x: 34.941177,
                max_y: 45.652176,
                min_x: 29.117645,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(163),
            Item::Rect(Rect {
                max_x: 34.941177,
                max_y: 45.652176,
                min_x: 34.941177,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(164),
            Item::Rect(Rect {
                max_x: 40.764706,
                max_y: 50.217392,
                min_x: 34.941177,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(165),
            Item::Rect(Rect {
                max_x: 46.588234,
                max_y: 54.782608,
                min_x: 40.764706,
                min_y: 50.217392,
            }),
        ),
        (
            ItemId(166),
            Item::Rect(Rect {
                max_x: 46.588234,
                max_y: 57.826088,
                min_x: 40.764706,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(167),
            Item::Rect(Rect {
                max_x: 40.764706,
                max_y: 57.826088,
                min_x: 34.941177,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(168),
            Item::Rect(Rect {
                max_x: 52.411762,
                max_y: 136.95653,
                min_x: 52.411762,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(169),
            Item::Rect(Rect {
                max_x: 52.411762,
                max_y: 136.95653,
                min_x: 52.411762,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(170),
            Item::Rect(Rect {
                max_x: 58.23529,
                max_y: 136.95653,
                min_x: 52.411762,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(171),
            Item::Rect(Rect {
                max_x: 58.23529,
                max_y: 127.82609,
                min_x: 58.23529,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(172),
            Item::Rect(Rect {
                max_x: 64.05882,
                max_y: 127.82609,
                min_x: 58.23529,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(173),
            Item::Rect(Rect {
                max_x: 69.882355,
                max_y: 127.82609,
                min_x: 64.05882,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(174),
            Item::Rect(Rect {
                max_x: 69.882355,
                max_y: 127.82609,
                min_x: 69.882355,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(175),
            Item::Rect(Rect {
                max_x: 69.882355,
                max_y: 127.82609,
                min_x: 69.882355,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(176),
            Item::Rect(Rect {
                max_x: 69.882355,
                max_y: 136.95653,
                min_x: 64.05882,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(177),
            Item::Rect(Rect {
                max_x: 64.05882,
                max_y: 146.08696,
                min_x: 58.23529,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(178),
            Item::Rect(Rect {
                max_x: 58.23529,
                max_y: 146.08696,
                min_x: 52.411762,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(179),
            Item::Rect(Rect {
                max_x: 168.88235,
                max_y: 146.08696,
                min_x: 168.88235,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(180),
            Item::Rect(Rect {
                max_x: 168.88235,
                max_y: 146.08696,
                min_x: 163.05882,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(181),
            Item::Rect(Rect {
                max_x: 168.88235,
                max_y: 136.95653,
                min_x: 163.05882,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(182),
            Item::Rect(Rect {
                max_x: 168.88235,
                max_y: 136.95653,
                min_x: 168.88235,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(183),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 18.26087,
                min_x: 232.94116,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(184),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 18.26087,
                min_x: 232.94116,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(185),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 9.130435,
                min_x: 232.94116,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(186),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 18.26087,
                min_x: 232.94116,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(187),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 18.26087,
                min_x: 232.94116,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(188),
            Item::Rect(Rect {
                max_x: 274.67648,
                max_y: 27.717392,
                min_x: 273.70587,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(189),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 27.717392,
                min_x: 273.4819,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(190),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 27.391304,
                min_x: 273.4819,
                min_y: 22.826088,
            }),
        ),
        (
            ItemId(191),
            Item::Rect(Rect {
                max_x: 274.67648,
                max_y: 27.391304,
                min_x: 273.70587,
                min_y: 22.826088,
            }),
        ),
        (
            ItemId(192),
            Item::Rect(Rect {
                max_x: 116.47058,
                max_y: 141.52174,
                min_x: 110.64706,
                min_y: 140.0,
            }),
        ),
        (
            ItemId(193),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 141.52174,
                min_x: 104.823524,
                min_y: 140.0,
            }),
        ),
        (
            ItemId(194),
            Item::Rect(Rect {
                max_x: 104.823524,
                max_y: 140.0,
                min_x: 99.0,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(195),
            Item::Rect(Rect {
                max_x: 99.0,
                max_y: 136.95653,
                min_x: 96.088234,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(196),
            Item::Rect(Rect {
                max_x: 99.0,
                max_y: 127.82609,
                min_x: 96.088234,
                min_y: 125.54348,
            }),
        ),
        (
            ItemId(197),
            Item::Rect(Rect {
                max_x: 103.367645,
                max_y: 125.54348,
                min_x: 99.0,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(198),
            Item::Rect(Rect {
                max_x: 103.367645,
                max_y: 118.695656,
                min_x: 100.94118,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(199),
            Item::Rect(Rect {
                max_x: 100.94118,
                max_y: 109.565216,
                min_x: 99.0,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(200),
            Item::Rect(Rect {
                max_x: 99.0,
                max_y: 100.434784,
                min_x: 99.0,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(201),
            Item::Rect(Rect {
                max_x: 99.0,
                max_y: 100.434784,
                min_x: 99.0,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(202),
            Item::Rect(Rect {
                max_x: 104.823524,
                max_y: 100.434784,
                min_x: 99.0,
                min_y: 95.86957,
            }),
        ),
        (
            ItemId(203),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 100.434784,
                min_x: 104.823524,
                min_y: 95.86957,
            }),
        ),
        (
            ItemId(204),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 100.434784,
                min_x: 110.64706,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(205),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 100.434784,
                min_x: 110.64706,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(206),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 109.565216,
                min_x: 105.199234,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(207),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 118.1066,
                min_x: 105.199234,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(208),
            Item::Rect(Rect {
                max_x: 116.276474,
                max_y: 118.1066,
                min_x: 110.64706,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(209),
            Item::Rect(Rect {
                max_x: 116.47058,
                max_y: 109.565216,
                min_x: 116.276474,
                min_y: 105.0,
            }),
        ),
        (
            ItemId(210),
            Item::Rect(Rect {
                max_x: 119.38235,
                max_y: 109.565216,
                min_x: 116.47058,
                min_y: 105.0,
            }),
        ),
        (
            ItemId(211),
            Item::Rect(Rect {
                max_x: 119.38235,
                max_y: 118.695656,
                min_x: 116.47058,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(212),
            Item::Rect(Rect {
                max_x: 122.29411,
                max_y: 124.782616,
                min_x: 116.47058,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(213),
            Item::Rect(Rect {
                max_x: 128.11765,
                max_y: 127.82609,
                min_x: 122.29411,
                min_y: 124.782616,
            }),
        ),
        (
            ItemId(214),
            Item::Rect(Rect {
                max_x: 128.11765,
                max_y: 127.82609,
                min_x: 128.11765,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(215),
            Item::Rect(Rect {
                max_x: 128.11765,
                max_y: 127.82609,
                min_x: 128.11765,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(216),
            Item::Rect(Rect {
                max_x: 128.11765,
                max_y: 136.95653,
                min_x: 125.20588,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(217),
            Item::Rect(Rect {
                max_x: 125.20588,
                max_y: 140.0,
                min_x: 122.29411,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(218),
            Item::Rect(Rect {
                max_x: 122.29411,
                max_y: 140.0,
                min_x: 116.47058,
                min_y: 140.0,
            }),
        ),
        (
            ItemId(219),
            Item::Rect(Rect {
                max_x: 139.76471,
                max_y: 173.47827,
                min_x: 133.94118,
                min_y: 168.91304,
            }),
        ),
        (
            ItemId(220),
            Item::Rect(Rect {
                max_x: 145.58823,
                max_y: 173.47827,
                min_x: 139.76471,
                min_y: 168.91304,
            }),
        ),
        (
            ItemId(221),
            Item::Rect(Rect {
                max_x: 145.58823,
                max_y: 173.47827,
                min_x: 145.58823,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(222),
            Item::Rect(Rect {
                max_x: 151.41176,
                max_y: 179.56522,
                min_x: 145.58823,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(223),
            Item::Rect(Rect {
                max_x: 154.32353,
                max_y: 182.6087,
                min_x: 151.41176,
                min_y: 179.56522,
            }),
        ),
        (
            ItemId(224),
            Item::Rect(Rect {
                max_x: 154.32353,
                max_y: 187.17392,
                min_x: 151.41176,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(225),
            Item::Rect(Rect {
                max_x: 151.41176,
                max_y: 191.73914,
                min_x: 145.58823,
                min_y: 187.17392,
            }),
        ),
        (
            ItemId(226),
            Item::Rect(Rect {
                max_x: 145.58823,
                max_y: 191.73914,
                min_x: 139.76471,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(227),
            Item::Rect(Rect {
                max_x: 139.76471,
                max_y: 182.6087,
                min_x: 133.94118,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(228),
            Item::Rect(Rect {
                max_x: 133.94118,
                max_y: 182.6087,
                min_x: 133.94118,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(229),
            Item::Rect(Rect {
                max_x: 133.94118,
                max_y: 173.47827,
                min_x: 133.94118,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(230),
            Item::Rect(Rect {
                max_x: 61.147057,
                max_y: 164.34782,
                min_x: 58.23529,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(231),
            Item::Rect(Rect {
                max_x: 58.23529,
                max_y: 164.34782,
                min_x: 55.32353,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(232),
            Item::Rect(Rect {
                max_x: 58.23529,
                max_y: 155.21739,
                min_x: 55.32353,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(233),
            Item::Rect(Rect {
                max_x: 61.147057,
                max_y: 155.21739,
                min_x: 58.23529,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(234),
            Item::Rect(Rect {
                max_x: 195.08823,
                max_y: 36.52174,
                min_x: 192.17647,
                min_y: 34.239132,
            }),
        ),
        (
            ItemId(235),
            Item::Rect(Rect {
                max_x: 195.08823,
                max_y: 41.086956,
                min_x: 192.17647,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(236),
            Item::Rect(Rect {
                max_x: 192.17647,
                max_y: 41.086956,
                min_x: 186.35294,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(237),
            Item::Rect(Rect {
                max_x: 186.35294,
                max_y: 36.52174,
                min_x: 186.35294,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(238),
            Item::Rect(Rect {
                max_x: 186.35294,
                max_y: 36.52174,
                min_x: 186.35294,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(239),
            Item::Rect(Rect {
                max_x: 192.17647,
                max_y: 36.52174,
                min_x: 186.35294,
                min_y: 34.239132,
            }),
        ),
        (
            ItemId(240),
            Item::Rect(Rect {
                max_x: 87.352936,
                max_y: 77.608696,
                min_x: 81.52941,
                min_y: 76.08695,
            }),
        ),
        (
            ItemId(241),
            Item::Rect(Rect {
                max_x: 81.52941,
                max_y: 76.08695,
                min_x: 78.617645,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(242),
            Item::Rect(Rect {
                max_x: 81.52941,
                max_y: 73.04348,
                min_x: 78.617645,
                min_y: 70.0,
            }),
        ),
        (
            ItemId(243),
            Item::Rect(Rect {
                max_x: 87.352936,
                max_y: 70.0,
                min_x: 81.52941,
                min_y: 68.478264,
            }),
        ),
        (
            ItemId(244),
            Item::Rect(Rect {
                max_x: 93.17647,
                max_y: 73.04348,
                min_x: 87.352936,
                min_y: 68.478264,
            }),
        ),
        (
            ItemId(245),
            Item::Rect(Rect {
                max_x: 93.17647,
                max_y: 73.04348,
                min_x: 93.17647,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(246),
            Item::Rect(Rect {
                max_x: 93.17647,
                max_y: 73.04348,
                min_x: 93.17647,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(247),
            Item::Rect(Rect {
                max_x: 93.17647,
                max_y: 77.608696,
                min_x: 87.352936,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(248),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 182.6087,
                min_x: 262.0588,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(249),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 182.6087,
                min_x: 262.0588,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(250),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 182.6087,
                min_x: 262.0588,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(251),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 182.6087,
                min_x: 262.0588,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(252),
            Item::Rect(Rect {
                max_x: 168.88235,
                max_y: 146.08696,
                min_x: 168.88235,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(253),
            Item::Rect(Rect {
                max_x: 168.88235,
                max_y: 146.08696,
                min_x: 168.88235,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(254),
            Item::Rect(Rect {
                max_x: 104.823524,
                max_y: 191.73914,
                min_x: 104.823524,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(255),
            Item::Rect(Rect {
                max_x: 104.823524,
                max_y: 191.73914,
                min_x: 104.823524,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(256),
            Item::Rect(Rect {
                max_x: 104.823524,
                max_y: 191.73914,
                min_x: 104.823524,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(257),
            Item::Rect(Rect {
                max_x: 104.823524,
                max_y: 191.73914,
                min_x: 104.823524,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(258),
            Item::Rect(Rect {
                max_x: 87.352936,
                max_y: 18.26087,
                min_x: 87.352936,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(259),
            Item::Rect(Rect {
                max_x: 87.352936,
                max_y: 27.391304,
                min_x: 87.352936,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(260),
            Item::Rect(Rect {
                max_x: 46.588234,
                max_y: 109.565216,
                min_x: 43.676468,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(261),
            Item::Rect(Rect {
                max_x: 49.5,
                max_y: 109.565216,
                min_x: 46.588234,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(262),
            Item::Rect(Rect {
                max_x: 49.5,
                max_y: 118.695656,
                min_x: 46.588234,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(263),
            Item::Rect(Rect {
                max_x: 46.588234,
                max_y: 127.82609,
                min_x: 40.764706,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(264),
            Item::Rect(Rect {
                max_x: 40.764706,
                max_y: 127.82609,
                min_x: 34.941177,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(265),
            Item::Rect(Rect {
                max_x: 40.764706,
                max_y: 118.695656,
                min_x: 34.941177,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(266),
            Item::Rect(Rect {
                max_x: 43.676468,
                max_y: 118.695656,
                min_x: 40.764706,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(267),
            Item::Rect(Rect {
                max_x: 34.941177,
                max_y: 146.08696,
                min_x: 34.941177,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(268),
            Item::Rect(Rect {
                max_x: 34.941177,
                max_y: 146.08696,
                min_x: 34.941177,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(269),
            Item::Rect(Rect {
                max_x: 34.941177,
                max_y: 146.08696,
                min_x: 34.941177,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(270),
            Item::Rect(Rect {
                max_x: 34.941177,
                max_y: 146.08696,
                min_x: 34.941177,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(271),
            Item::Rect(Rect {
                max_x: 268.10632,
                max_y: 63.913044,
                min_x: 267.88235,
                min_y: 63.561874,
            }),
        ),
        (
            ItemId(272),
            Item::Rect(Rect {
                max_x: 268.10632,
                max_y: 66.95653,
                min_x: 267.88235,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(273),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 66.95653,
                min_x: 266.71762,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(274),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 63.913044,
                min_x: 266.71762,
                min_y: 63.561874,
            }),
        ),
        (
            ItemId(275),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 164.34782,
                min_x: 227.11765,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(276),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 164.34782,
                min_x: 227.11765,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(277),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 164.34782,
                min_x: 227.11765,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(278),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 164.34782,
                min_x: 227.11765,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(279),
            Item::Rect(Rect {
                max_x: 17.470589,
                max_y: 54.782608,
                min_x: 17.470589,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(280),
            Item::Rect(Rect {
                max_x: 198.0,
                max_y: 18.26087,
                min_x: 198.0,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(281),
            Item::Rect(Rect {
                max_x: 198.0,
                max_y: 18.26087,
                min_x: 192.17647,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(282),
            Item::Rect(Rect {
                max_x: 192.17647,
                max_y: 9.130435,
                min_x: 192.17647,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(283),
            Item::Rect(Rect {
                max_x: 192.17647,
                max_y: 9.130435,
                min_x: 192.17647,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(284),
            Item::Rect(Rect {
                max_x: 198.0,
                max_y: 9.130435,
                min_x: 192.17647,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(285),
            Item::Rect(Rect {
                max_x: 198.0,
                max_y: 9.130435,
                min_x: 198.0,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(286),
            Item::Rect(Rect {
                max_x: 248.47058,
                max_y: 45.652176,
                min_x: 247.5,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(287),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 36.52174,
                min_x: 247.5,
                min_y: 35.892056,
            }),
        ),
        (
            ItemId(288),
            Item::Rect(Rect {
                max_x: 250.84312,
                max_y: 36.52174,
                min_x: 250.41176,
                min_y: 35.892056,
            }),
        ),
        (
            ItemId(289),
            Item::Rect(Rect {
                max_x: 250.84312,
                max_y: 45.652176,
                min_x: 250.61256,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(290),
            Item::Rect(Rect {
                max_x: 250.61256,
                max_y: 48.695656,
                min_x: 250.41176,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(291),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 48.695656,
                min_x: 248.47058,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(292),
            Item::Rect(Rect {
                max_x: 34.941177,
                max_y: 118.695656,
                min_x: 29.117645,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(293),
            Item::Rect(Rect {
                max_x: 29.117645,
                max_y: 118.695656,
                min_x: 29.117645,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(294),
            Item::Rect(Rect {
                max_x: 29.117645,
                max_y: 118.695656,
                min_x: 29.117645,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(295),
            Item::Rect(Rect {
                max_x: 34.941177,
                max_y: 118.695656,
                min_x: 29.117645,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(296),
            Item::Rect(Rect {
                max_x: 151.41176,
                max_y: 118.695656,
                min_x: 151.41176,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(297),
            Item::Rect(Rect {
                max_x: 151.41176,
                max_y: 118.695656,
                min_x: 151.41176,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(298),
            Item::Rect(Rect {
                max_x: 151.41176,
                max_y: 118.695656,
                min_x: 151.41176,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(299),
            Item::Rect(Rect {
                max_x: 151.41176,
                max_y: 118.695656,
                min_x: 151.41176,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(300),
            Item::Rect(Rect {
                max_x: 133.94118,
                max_y: 182.6087,
                min_x: 133.94118,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(301),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 36.52174,
                min_x: 279.52942,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(302),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 36.52174,
                min_x: 279.52942,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(303),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 36.52174,
                min_x: 279.52942,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(304),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 36.52174,
                min_x: 279.52942,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(305),
            Item::Rect(Rect {
                max_x: 93.17647,
                max_y: 41.086956,
                min_x: 90.2647,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(306),
            Item::Rect(Rect {
                max_x: 93.17647,
                max_y: 36.52174,
                min_x: 90.2647,
                min_y: 31.956522,
            }),
        ),
        (
            ItemId(307),
            Item::Rect(Rect {
                max_x: 99.0,
                max_y: 36.52174,
                min_x: 93.17647,
                min_y: 31.956522,
            }),
        ),
        (
            ItemId(308),
            Item::Rect(Rect {
                max_x: 99.0,
                max_y: 36.52174,
                min_x: 99.0,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(309),
            Item::Rect(Rect {
                max_x: 99.0,
                max_y: 36.52174,
                min_x: 99.0,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(310),
            Item::Rect(Rect {
                max_x: 99.0,
                max_y: 41.086956,
                min_x: 93.17647,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(311),
            Item::Rect(Rect {
                max_x: 34.941177,
                max_y: 18.26087,
                min_x: 34.941177,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(312),
            Item::Rect(Rect {
                max_x: 52.411762,
                max_y: 182.6087,
                min_x: 52.411762,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(313),
            Item::Rect(Rect {
                max_x: 52.411762,
                max_y: 182.6087,
                min_x: 52.411762,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(314),
            Item::Rect(Rect {
                max_x: 52.411762,
                max_y: 182.6087,
                min_x: 52.411762,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(315),
            Item::Rect(Rect {
                max_x: 52.411762,
                max_y: 182.6087,
                min_x: 52.411762,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(316),
            Item::Rect(Rect {
                max_x: 99.0,
                max_y: 136.95653,
                min_x: 99.0,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(317),
            Item::Rect(Rect {
                max_x: 157.23529,
                max_y: 36.52174,
                min_x: 157.23529,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(318),
            Item::Rect(Rect {
                max_x: 157.23529,
                max_y: 36.52174,
                min_x: 157.23529,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(319),
            Item::Rect(Rect {
                max_x: 163.05882,
                max_y: 36.52174,
                min_x: 157.23529,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(320),
            Item::Rect(Rect {
                max_x: 163.05882,
                max_y: 36.52174,
                min_x: 163.05882,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(321),
            Item::Rect(Rect {
                max_x: 163.05882,
                max_y: 36.52174,
                min_x: 163.05882,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(322),
            Item::Rect(Rect {
                max_x: 163.05882,
                max_y: 36.52174,
                min_x: 157.23529,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(323),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 45.652176,
                min_x: 11.6470585,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(324),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 182.6087,
                min_x: 250.41176,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(325),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 182.6087,
                min_x: 250.41176,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(326),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 182.6087,
                min_x: 250.41176,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(327),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 182.6087,
                min_x: 250.41176,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(328),
            Item::Rect(Rect {
                max_x: 198.0,
                max_y: 18.26087,
                min_x: 198.0,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(329),
            Item::Rect(Rect {
                max_x: 198.0,
                max_y: 18.26087,
                min_x: 198.0,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(330),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 127.82609,
                min_x: 11.6470585,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(331),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 127.82609,
                min_x: 11.6470585,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(332),
            Item::Rect(Rect {
                max_x: 17.470589,
                max_y: 127.82609,
                min_x: 11.6470585,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(333),
            Item::Rect(Rect {
                max_x: 17.470589,
                max_y: 127.82609,
                min_x: 17.470589,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(334),
            Item::Rect(Rect {
                max_x: 17.470589,
                max_y: 127.82609,
                min_x: 17.470589,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(335),
            Item::Rect(Rect {
                max_x: 17.470589,
                max_y: 127.82609,
                min_x: 11.6470585,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(336),
            Item::Rect(Rect {
                max_x: 139.76471,
                max_y: 200.86957,
                min_x: 136.85294,
                min_y: 196.30435,
            }),
        ),
        (
            ItemId(337),
            Item::Rect(Rect {
                max_x: 145.58823,
                max_y: 196.30435,
                min_x: 139.76471,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(338),
            Item::Rect(Rect {
                max_x: 145.58823,
                max_y: 200.86957,
                min_x: 145.58823,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(339),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 109.565216,
                min_x: 285.35294,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(340),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 109.565216,
                min_x: 285.35294,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(341),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 109.565216,
                min_x: 285.35294,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(342),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 109.565216,
                min_x: 285.35294,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(343),
            Item::Rect(Rect {
                max_x: 17.470589,
                max_y: 27.391304,
                min_x: 17.470589,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(344),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 9.130435,
                min_x: 232.94116,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(345),
            Item::Rect(Rect {
                max_x: 279.9608,
                max_y: 88.26087,
                min_x: 279.52942,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(346),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 88.26087,
                min_x: 277.58823,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(347),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 82.17391,
                min_x: 277.58823,
                min_y: 81.49759,
            }),
        ),
        (
            ItemId(348),
            Item::Rect(Rect {
                max_x: 279.9608,
                max_y: 82.17391,
                min_x: 279.52942,
                min_y: 81.49759,
            }),
        ),
        (
            ItemId(349),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 82.17391,
                min_x: 110.64706,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(350),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 82.17391,
                min_x: 110.64706,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(351),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 82.17391,
                min_x: 110.64706,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(352),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 82.17391,
                min_x: 110.64706,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(353),
            Item::Rect(Rect {
                max_x: 40.764706,
                max_y: 127.82609,
                min_x: 40.764706,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(354),
            Item::Rect(Rect {
                max_x: 40.764706,
                max_y: 127.82609,
                min_x: 40.764706,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(355),
            Item::Rect(Rect {
                max_x: 93.17647,
                max_y: 54.782608,
                min_x: 93.17647,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(356),
            Item::Rect(Rect {
                max_x: 93.17647,
                max_y: 54.782608,
                min_x: 93.17647,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(357),
            Item::Rect(Rect {
                max_x: 93.17647,
                max_y: 54.782608,
                min_x: 93.17647,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(358),
            Item::Rect(Rect {
                max_x: 93.17647,
                max_y: 54.782608,
                min_x: 93.17647,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(359),
            Item::Rect(Rect {
                max_x: 163.05882,
                max_y: 136.95653,
                min_x: 157.23529,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(360),
            Item::Rect(Rect {
                max_x: 157.23529,
                max_y: 136.95653,
                min_x: 157.23529,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(361),
            Item::Rect(Rect {
                max_x: 157.23529,
                max_y: 136.95653,
                min_x: 157.23529,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(362),
            Item::Rect(Rect {
                max_x: 163.05882,
                max_y: 136.95653,
                min_x: 157.23529,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(363),
            Item::Rect(Rect {
                max_x: 64.05882,
                max_y: 136.95653,
                min_x: 64.05882,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(364),
            Item::Rect(Rect {
                max_x: 46.588234,
                max_y: 118.695656,
                min_x: 46.588234,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(365),
            Item::Rect(Rect {
                max_x: 58.23529,
                max_y: 164.34782,
                min_x: 58.23529,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(366),
            Item::Rect(Rect {
                max_x: 58.23529,
                max_y: 164.34782,
                min_x: 58.23529,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(367),
            Item::Rect(Rect {
                max_x: 198.0,
                max_y: 4.5652175,
                min_x: 194.11763,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(368),
            Item::Rect(Rect {
                max_x: 201.88235,
                max_y: 4.5652175,
                min_x: 198.0,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(369),
            Item::Rect(Rect {
                max_x: 157.23529,
                max_y: 9.130435,
                min_x: 151.41176,
                min_y: 4.5652175,
            }),
        ),
        (
            ItemId(370),
            Item::Rect(Rect {
                max_x: 157.23529,
                max_y: 9.130435,
                min_x: 157.23529,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(371),
            Item::Rect(Rect {
                max_x: 157.23529,
                max_y: 18.26087,
                min_x: 157.23529,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(372),
            Item::Rect(Rect {
                max_x: 157.23529,
                max_y: 22.826088,
                min_x: 151.41176,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(373),
            Item::Rect(Rect {
                max_x: 151.41176,
                max_y: 22.826088,
                min_x: 148.5,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(374),
            Item::Rect(Rect {
                max_x: 148.5,
                max_y: 18.26087,
                min_x: 145.58823,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(375),
            Item::Rect(Rect {
                max_x: 145.58823,
                max_y: 9.130435,
                min_x: 145.58823,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(376),
            Item::Rect(Rect {
                max_x: 151.41176,
                max_y: 4.5652175,
                min_x: 145.58823,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(377),
            Item::Rect(Rect {
                max_x: 16.694118,
                max_y: 7.9130435,
                min_x: 11.6470585,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(378),
            Item::Rect(Rect {
                max_x: 163.05882,
                max_y: 0.0,
                min_x: 163.05882,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(379),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 1.0144928,
                min_x: 255.56335,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(380),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 7.9130435,
                min_x: 5.8235292,
                min_y: 7.826087,
            }),
        ),
        (
            ItemId(381),
            Item::Rect(Rect {
                max_x: 128.11765,
                max_y: 0.0,
                min_x: 128.11765,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(382),
            Item::Rect(Rect {
                max_x: 128.11765,
                max_y: 0.0,
                min_x: 128.11765,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(383),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 155.21739,
                min_x: 2.9117646,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(384),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 146.08696,
                min_x: 5.8235292,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(385),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 146.08696,
                min_x: 5.8235292,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(386),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 136.95653,
                min_x: 5.8235292,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(387),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 127.82609,
                min_x: 5.8235292,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(388),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 118.695656,
                min_x: 5.8235292,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(389),
            Item::Rect(Rect {
                max_x: 8.735294,
                max_y: 118.695656,
                min_x: 5.8235292,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(390),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 109.565216,
                min_x: 8.735294,
                min_y: 105.0,
            }),
        ),
        (
            ItemId(391),
            Item::Rect(Rect {
                max_x: 17.470589,
                max_y: 109.565216,
                min_x: 11.6470585,
                min_y: 105.0,
            }),
        ),
        (
            ItemId(392),
            Item::Rect(Rect {
                max_x: 23.294117,
                max_y: 109.565216,
                min_x: 17.470589,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(393),
            Item::Rect(Rect {
                max_x: 29.117645,
                max_y: 109.565216,
                min_x: 23.294117,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(394),
            Item::Rect(Rect {
                max_x: 34.941177,
                max_y: 109.565216,
                min_x: 29.117645,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(395),
            Item::Rect(Rect {
                max_x: 34.941177,
                max_y: 100.434784,
                min_x: 34.941177,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(396),
            Item::Rect(Rect {
                max_x: 40.764706,
                max_y: 100.434784,
                min_x: 34.941177,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(397),
            Item::Rect(Rect {
                max_x: 40.764706,
                max_y: 91.30435,
                min_x: 34.941177,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(398),
            Item::Rect(Rect {
                max_x: 34.941177,
                max_y: 82.17391,
                min_x: 32.02941,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(399),
            Item::Rect(Rect {
                max_x: 34.941177,
                max_y: 73.04348,
                min_x: 32.02941,
                min_y: 68.478264,
            }),
        ),
        (
            ItemId(400),
            Item::Rect(Rect {
                max_x: 40.764706,
                max_y: 68.478264,
                min_x: 34.941177,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(401),
            Item::Rect(Rect {
                max_x: 40.764706,
                max_y: 63.913044,
                min_x: 34.941177,
                min_y: 60.869564,
            }),
        ),
        (
            ItemId(402),
            Item::Rect(Rect {
                max_x: 34.941177,
                max_y: 63.913044,
                min_x: 29.117645,
                min_y: 60.869564,
            }),
        ),
        (
            ItemId(403),
            Item::Rect(Rect {
                max_x: 29.117645,
                max_y: 63.913044,
                min_x: 23.294117,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(404),
            Item::Rect(Rect {
                max_x: 23.294117,
                max_y: 63.913044,
                min_x: 17.470589,
                min_y: 59.347828,
            }),
        ),
        (
            ItemId(405),
            Item::Rect(Rect {
                max_x: 17.470589,
                max_y: 59.347828,
                min_x: 11.6470585,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(406),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 54.782608,
                min_x: 5.8235292,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(407),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 54.782608,
                min_x: 0.0,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(408),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 63.913044,
                min_x: 0.0,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(409),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 63.913044,
                min_x: 5.8235292,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(410),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 63.913044,
                min_x: 5.8235292,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(411),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 63.913044,
                min_x: 0.0,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(412),
            Item::Rect(Rect {
                max_x: 221.29411,
                max_y: 0.0,
                min_x: 221.29411,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(413),
            Item::Rect(Rect {
                max_x: 221.29411,
                max_y: 4.5652175,
                min_x: 215.47058,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(414),
            Item::Rect(Rect {
                max_x: 215.47058,
                max_y: 9.130435,
                min_x: 209.64705,
                min_y: 4.5652175,
            }),
        ),
        (
            ItemId(415),
            Item::Rect(Rect {
                max_x: 209.64705,
                max_y: 18.26087,
                min_x: 205.7647,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(416),
            Item::Rect(Rect {
                max_x: 209.64705,
                max_y: 27.391304,
                min_x: 205.7647,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(417),
            Item::Rect(Rect {
                max_x: 209.64705,
                max_y: 27.391304,
                min_x: 209.64705,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(418),
            Item::Rect(Rect {
                max_x: 215.47058,
                max_y: 31.956522,
                min_x: 209.64705,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(419),
            Item::Rect(Rect {
                max_x: 218.38234,
                max_y: 36.52174,
                min_x: 215.47058,
                min_y: 31.956522,
            }),
        ),
        (
            ItemId(420),
            Item::Rect(Rect {
                max_x: 221.29411,
                max_y: 41.086956,
                min_x: 218.38234,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(421),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 45.652176,
                min_x: 221.29411,
                min_y: 41.086956,
            }),
        ),
        (
            ItemId(422),
            Item::Rect(Rect {
                max_x: 230.0294,
                max_y: 45.652176,
                min_x: 227.11765,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(423),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 36.52174,
                min_x: 230.0294,
                min_y: 33.478264,
            }),
        ),
        (
            ItemId(424),
            Item::Rect(Rect {
                max_x: 238.7647,
                max_y: 36.52174,
                min_x: 232.94116,
                min_y: 33.478264,
            }),
        ),
        (
            ItemId(425),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 36.52174,
                min_x: 238.7647,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(426),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 36.52174,
                min_x: 244.58823,
                min_y: 35.262367,
            }),
        ),
        (
            ItemId(427),
            Item::Rect(Rect {
                max_x: 251.2745,
                max_y: 36.52174,
                min_x: 250.41176,
                min_y: 35.262367,
            }),
        ),
        (
            ItemId(428),
            Item::Rect(Rect {
                max_x: 251.2745,
                max_y: 45.652176,
                min_x: 251.01419,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(429),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 54.444447,
                min_x: 251.01419,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(430),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 54.444447,
                min_x: 256.2353,
                min_y: 53.429955,
            }),
        ),
        (
            ItemId(431),
            Item::Rect(Rect {
                max_x: 262.92157,
                max_y: 54.782608,
                min_x: 262.0588,
                min_y: 53.429955,
            }),
        ),
        (
            ItemId(432),
            Item::Rect(Rect {
                max_x: 262.92157,
                max_y: 60.869564,
                min_x: 262.0588,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(433),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 60.869564,
                min_x: 256.2353,
                min_y: 57.826088,
            }),
        ),
        (
            ItemId(434),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 57.826088,
                min_x: 250.41176,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(435),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 54.782608,
                min_x: 244.58823,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(436),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 45.652176,
                min_x: 238.7647,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(437),
            Item::Rect(Rect {
                max_x: 238.7647,
                max_y: 45.652176,
                min_x: 232.94116,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(438),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 54.782608,
                min_x: 230.0294,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(439),
            Item::Rect(Rect {
                max_x: 230.0294,
                max_y: 57.826088,
                min_x: 227.11765,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(440),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 57.826088,
                min_x: 221.29411,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(441),
            Item::Rect(Rect {
                max_x: 221.29411,
                max_y: 54.782608,
                min_x: 215.47058,
                min_y: 48.695656,
            }),
        ),
        (
            ItemId(442),
            Item::Rect(Rect {
                max_x: 215.47058,
                max_y: 50.217392,
                min_x: 209.64705,
                min_y: 48.695656,
            }),
        ),
        (
            ItemId(443),
            Item::Rect(Rect {
                max_x: 209.64705,
                max_y: 54.782608,
                min_x: 203.82352,
                min_y: 50.217392,
            }),
        ),
        (
            ItemId(444),
            Item::Rect(Rect {
                max_x: 203.82352,
                max_y: 54.782608,
                min_x: 198.0,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(445),
            Item::Rect(Rect {
                max_x: 198.0,
                max_y: 54.782608,
                min_x: 192.17647,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(446),
            Item::Rect(Rect {
                max_x: 198.0,
                max_y: 63.913044,
                min_x: 192.17647,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(447),
            Item::Rect(Rect {
                max_x: 203.82352,
                max_y: 63.913044,
                min_x: 198.0,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(448),
            Item::Rect(Rect {
                max_x: 209.64705,
                max_y: 63.913044,
                min_x: 203.82352,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(449),
            Item::Rect(Rect {
                max_x: 215.47058,
                max_y: 63.913044,
                min_x: 209.64705,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(450),
            Item::Rect(Rect {
                max_x: 215.47058,
                max_y: 63.913044,
                min_x: 215.47058,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(451),
            Item::Rect(Rect {
                max_x: 215.47058,
                max_y: 73.04348,
                min_x: 215.47058,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(452),
            Item::Rect(Rect {
                max_x: 215.47058,
                max_y: 77.608696,
                min_x: 209.64705,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(453),
            Item::Rect(Rect {
                max_x: 209.64705,
                max_y: 77.608696,
                min_x: 203.82352,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(454),
            Item::Rect(Rect {
                max_x: 203.82352,
                max_y: 73.04348,
                min_x: 198.0,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(455),
            Item::Rect(Rect {
                max_x: 198.0,
                max_y: 63.913044,
                min_x: 192.17647,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(456),
            Item::Rect(Rect {
                max_x: 192.17647,
                max_y: 63.913044,
                min_x: 186.35294,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(457),
            Item::Rect(Rect {
                max_x: 186.35294,
                max_y: 63.913044,
                min_x: 180.5294,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(458),
            Item::Rect(Rect {
                max_x: 180.5294,
                max_y: 54.782608,
                min_x: 180.5294,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(459),
            Item::Rect(Rect {
                max_x: 180.5294,
                max_y: 54.782608,
                min_x: 180.5294,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(460),
            Item::Rect(Rect {
                max_x: 183.44118,
                max_y: 54.782608,
                min_x: 180.5294,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(461),
            Item::Rect(Rect {
                max_x: 183.44118,
                max_y: 45.652176,
                min_x: 180.5294,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(462),
            Item::Rect(Rect {
                max_x: 180.5294,
                max_y: 36.52174,
                min_x: 180.5294,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(463),
            Item::Rect(Rect {
                max_x: 180.5294,
                max_y: 36.52174,
                min_x: 180.5294,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(464),
            Item::Rect(Rect {
                max_x: 186.35294,
                max_y: 36.52174,
                min_x: 180.5294,
                min_y: 30.434782,
            }),
        ),
        (
            ItemId(465),
            Item::Rect(Rect {
                max_x: 192.17647,
                max_y: 30.434782,
                min_x: 186.35294,
                min_y: 29.673914,
            }),
        ),
        (
            ItemId(466),
            Item::Rect(Rect {
                max_x: 198.0,
                max_y: 31.956522,
                min_x: 192.17647,
                min_y: 29.673914,
            }),
        ),
        (
            ItemId(467),
            Item::Rect(Rect {
                max_x: 203.82352,
                max_y: 31.956522,
                min_x: 198.0,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(468),
            Item::Rect(Rect {
                max_x: 203.82352,
                max_y: 27.391304,
                min_x: 198.0,
                min_y: 24.347828,
            }),
        ),
        (
            ItemId(469),
            Item::Rect(Rect {
                max_x: 198.0,
                max_y: 24.347828,
                min_x: 192.17647,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(470),
            Item::Rect(Rect {
                max_x: 192.17647,
                max_y: 18.26087,
                min_x: 186.35294,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(471),
            Item::Rect(Rect {
                max_x: 186.35294,
                max_y: 18.26087,
                min_x: 180.5294,
                min_y: 12.173914,
            }),
        ),
        (
            ItemId(472),
            Item::Rect(Rect {
                max_x: 180.5294,
                max_y: 12.173914,
                min_x: 174.70587,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(473),
            Item::Rect(Rect {
                max_x: 174.70587,
                max_y: 9.130435,
                min_x: 168.88235,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(474),
            Item::Rect(Rect {
                max_x: 168.88235,
                max_y: 0.0,
                min_x: 163.05882,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(475),
            Item::Rect(Rect {
                max_x: 106.76471,
                max_y: 4.5652175,
                min_x: 104.823524,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(476),
            Item::Rect(Rect {
                max_x: 104.823524,
                max_y: 9.130435,
                min_x: 99.0,
                min_y: 4.5652175,
            }),
        ),
        (
            ItemId(477),
            Item::Rect(Rect {
                max_x: 99.0,
                max_y: 18.26087,
                min_x: 99.0,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(478),
            Item::Rect(Rect {
                max_x: 104.823524,
                max_y: 27.391304,
                min_x: 99.0,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(479),
            Item::Rect(Rect {
                max_x: 104.823524,
                max_y: 27.391304,
                min_x: 104.823524,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(480),
            Item::Rect(Rect {
                max_x: 104.823524,
                max_y: 27.391304,
                min_x: 104.823524,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(481),
            Item::Rect(Rect {
                max_x: 104.823524,
                max_y: 36.52174,
                min_x: 102.88235,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(482),
            Item::Rect(Rect {
                max_x: 102.88235,
                max_y: 45.652176,
                min_x: 99.0,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(483),
            Item::Rect(Rect {
                max_x: 99.0,
                max_y: 54.782608,
                min_x: 97.058815,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(484),
            Item::Rect(Rect {
                max_x: 99.0,
                max_y: 63.913044,
                min_x: 97.058815,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(485),
            Item::Rect(Rect {
                max_x: 99.0,
                max_y: 63.913044,
                min_x: 99.0,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(486),
            Item::Rect(Rect {
                max_x: 101.91176,
                max_y: 73.04348,
                min_x: 99.0,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(487),
            Item::Rect(Rect {
                max_x: 101.91176,
                max_y: 77.608696,
                min_x: 99.0,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(488),
            Item::Rect(Rect {
                max_x: 99.0,
                max_y: 82.17391,
                min_x: 96.088234,
                min_y: 77.608696,
            }),
        ),
        (
            ItemId(489),
            Item::Rect(Rect {
                max_x: 99.0,
                max_y: 91.30435,
                min_x: 96.088234,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(490),
            Item::Rect(Rect {
                max_x: 104.823524,
                max_y: 91.30435,
                min_x: 99.0,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(491),
            Item::Rect(Rect {
                max_x: 104.823524,
                max_y: 82.17391,
                min_x: 104.823524,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(492),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 82.17391,
                min_x: 104.823524,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(493),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 73.04348,
                min_x: 110.64706,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(494),
            Item::Rect(Rect {
                max_x: 116.47058,
                max_y: 77.608696,
                min_x: 110.64706,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(495),
            Item::Rect(Rect {
                max_x: 119.38235,
                max_y: 82.17391,
                min_x: 116.47058,
                min_y: 77.608696,
            }),
        ),
        (
            ItemId(496),
            Item::Rect(Rect {
                max_x: 119.38235,
                max_y: 91.30435,
                min_x: 116.47058,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(497),
            Item::Rect(Rect {
                max_x: 122.29411,
                max_y: 100.434784,
                min_x: 116.47058,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(498),
            Item::Rect(Rect {
                max_x: 128.11765,
                max_y: 100.434784,
                min_x: 122.29411,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(499),
            Item::Rect(Rect {
                max_x: 128.11765,
                max_y: 100.434784,
                min_x: 128.11765,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(500),
            Item::Rect(Rect {
                max_x: 133.94118,
                max_y: 109.565216,
                min_x: 128.11765,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(501),
            Item::Rect(Rect {
                max_x: 133.94118,
                max_y: 109.565216,
                min_x: 133.94118,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(502),
            Item::Rect(Rect {
                max_x: 139.76471,
                max_y: 114.13044,
                min_x: 133.94118,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(503),
            Item::Rect(Rect {
                max_x: 145.58823,
                max_y: 115.652176,
                min_x: 139.76471,
                min_y: 114.13044,
            }),
        ),
        (
            ItemId(504),
            Item::Rect(Rect {
                max_x: 151.41176,
                max_y: 115.652176,
                min_x: 145.58823,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(505),
            Item::Rect(Rect {
                max_x: 151.41176,
                max_y: 109.565216,
                min_x: 151.41176,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(506),
            Item::Rect(Rect {
                max_x: 151.41176,
                max_y: 100.434784,
                min_x: 145.58823,
                min_y: 95.86957,
            }),
        ),
        (
            ItemId(507),
            Item::Rect(Rect {
                max_x: 145.58823,
                max_y: 100.434784,
                min_x: 139.76471,
                min_y: 95.86957,
            }),
        ),
        (
            ItemId(508),
            Item::Rect(Rect {
                max_x: 139.76471,
                max_y: 100.434784,
                min_x: 133.94118,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(509),
            Item::Rect(Rect {
                max_x: 133.94118,
                max_y: 91.30435,
                min_x: 133.94118,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(510),
            Item::Rect(Rect {
                max_x: 133.94118,
                max_y: 91.30435,
                min_x: 133.94118,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(511),
            Item::Rect(Rect {
                max_x: 139.76471,
                max_y: 91.30435,
                min_x: 133.94118,
                min_y: 86.739136,
            }),
        ),
        (
            ItemId(512),
            Item::Rect(Rect {
                max_x: 145.58823,
                max_y: 86.739136,
                min_x: 139.76471,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(513),
            Item::Rect(Rect {
                max_x: 145.58823,
                max_y: 82.17391,
                min_x: 145.58823,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(514),
            Item::Rect(Rect {
                max_x: 151.41176,
                max_y: 82.17391,
                min_x: 145.58823,
                min_y: 77.608696,
            }),
        ),
        (
            ItemId(515),
            Item::Rect(Rect {
                max_x: 153.35294,
                max_y: 82.17391,
                min_x: 151.41176,
                min_y: 77.608696,
            }),
        ),
        (
            ItemId(516),
            Item::Rect(Rect {
                max_x: 153.35294,
                max_y: 91.30435,
                min_x: 151.41176,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(517),
            Item::Rect(Rect {
                max_x: 157.23529,
                max_y: 100.434784,
                min_x: 151.41176,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(518),
            Item::Rect(Rect {
                max_x: 157.23529,
                max_y: 100.434784,
                min_x: 157.23529,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(519),
            Item::Rect(Rect {
                max_x: 157.23529,
                max_y: 109.565216,
                min_x: 157.23529,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(520),
            Item::Rect(Rect {
                max_x: 159.17647,
                max_y: 118.695656,
                min_x: 157.23529,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(521),
            Item::Rect(Rect {
                max_x: 163.05882,
                max_y: 127.82609,
                min_x: 159.17647,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(522),
            Item::Rect(Rect {
                max_x: 163.05882,
                max_y: 127.82609,
                min_x: 163.05882,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(523),
            Item::Rect(Rect {
                max_x: 168.88235,
                max_y: 130.86957,
                min_x: 163.05882,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(524),
            Item::Rect(Rect {
                max_x: 174.70587,
                max_y: 136.95653,
                min_x: 168.88235,
                min_y: 130.86957,
            }),
        ),
        (
            ItemId(525),
            Item::Rect(Rect {
                max_x: 180.5294,
                max_y: 136.95653,
                min_x: 174.70587,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(526),
            Item::Rect(Rect {
                max_x: 186.35294,
                max_y: 136.95653,
                min_x: 180.5294,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(527),
            Item::Rect(Rect {
                max_x: 192.17647,
                max_y: 136.95653,
                min_x: 186.35294,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(528),
            Item::Rect(Rect {
                max_x: 198.0,
                max_y: 136.95653,
                min_x: 192.17647,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(529),
            Item::Rect(Rect {
                max_x: 203.82352,
                max_y: 136.95653,
                min_x: 198.0,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(530),
            Item::Rect(Rect {
                max_x: 203.82352,
                max_y: 127.82609,
                min_x: 203.82352,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(531),
            Item::Rect(Rect {
                max_x: 203.82352,
                max_y: 118.695656,
                min_x: 203.82352,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(532),
            Item::Rect(Rect {
                max_x: 203.82352,
                max_y: 109.565216,
                min_x: 203.82352,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(533),
            Item::Rect(Rect {
                max_x: 203.82352,
                max_y: 118.695656,
                min_x: 203.82352,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(534),
            Item::Rect(Rect {
                max_x: 209.64705,
                max_y: 127.82609,
                min_x: 203.82352,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(535),
            Item::Rect(Rect {
                max_x: 215.47058,
                max_y: 127.82609,
                min_x: 209.64705,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(536),
            Item::Rect(Rect {
                max_x: 221.29411,
                max_y: 127.82609,
                min_x: 215.47058,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(537),
            Item::Rect(Rect {
                max_x: 221.29411,
                max_y: 118.695656,
                min_x: 221.29411,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(538),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 118.695656,
                min_x: 221.29411,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(539),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 118.695656,
                min_x: 227.11765,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(540),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 123.26087,
                min_x: 227.11765,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(541),
            Item::Rect(Rect {
                max_x: 235.85294,
                max_y: 127.82609,
                min_x: 232.94116,
                min_y: 123.26087,
            }),
        ),
        (
            ItemId(542),
            Item::Rect(Rect {
                max_x: 235.85294,
                max_y: 136.95653,
                min_x: 235.85294,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(543),
            Item::Rect(Rect {
                max_x: 235.85294,
                max_y: 141.52174,
                min_x: 232.94116,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(544),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 146.08696,
                min_x: 230.0294,
                min_y: 141.52174,
            }),
        ),
        (
            ItemId(545),
            Item::Rect(Rect {
                max_x: 230.0294,
                max_y: 155.21739,
                min_x: 227.11765,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(546),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 164.34782,
                min_x: 227.11765,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(547),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 164.34782,
                min_x: 232.94116,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(548),
            Item::Rect(Rect {
                max_x: 238.7647,
                max_y: 173.47827,
                min_x: 232.94116,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(549),
            Item::Rect(Rect {
                max_x: 238.7647,
                max_y: 173.47827,
                min_x: 238.7647,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(550),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 182.6087,
                min_x: 238.7647,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(551),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 182.6087,
                min_x: 244.58823,
                min_y: 176.52174,
            }),
        ),
        (
            ItemId(552),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 178.04349,
                min_x: 250.41176,
                min_y: 176.52174,
            }),
        ),
        (
            ItemId(553),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 178.04349,
                min_x: 256.2353,
                min_y: 176.52174,
            }),
        ),
        (
            ItemId(554),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 179.56522,
                min_x: 262.0588,
                min_y: 176.52174,
            }),
        ),
        (
            ItemId(555),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 182.6087,
                min_x: 267.88235,
                min_y: 179.56522,
            }),
        ),
        (
            ItemId(556),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 182.6087,
                min_x: 273.70587,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(557),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 191.73914,
                min_x: 273.70587,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(558),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 191.73914,
                min_x: 279.52942,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(559),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 191.73914,
                min_x: 279.52942,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(560),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 200.86957,
                min_x: 273.70587,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(561),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 1.0144928,
                min_x: 256.2353,
                min_y: 0.62968516,
            }),
        ),
        (
            ItemId(562),
            Item::Rect(Rect {
                max_x: 0.0,
                max_y: 191.73914,
                min_x: 0.0,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(563),
            Item::Rect(Rect {
                max_x: 0.0,
                max_y: 191.73914,
                min_x: 0.0,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(564),
            Item::Rect(Rect {
                max_x: 233.37254,
                max_y: 9.130435,
                min_x: 232.94116,
                min_y: 8.400001,
            }),
        ),
        (
            ItemId(565),
            Item::Rect(Rect {
                max_x: 238.7647,
                max_y: 17.909698,
                min_x: 233.37254,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(566),
            Item::Rect(Rect {
                max_x: 238.97267,
                max_y: 18.26087,
                min_x: 238.7647,
                min_y: 17.909698,
            }),
        ),
        (
            ItemId(567),
            Item::Rect(Rect {
                max_x: 238.97267,
                max_y: 20.543478,
                min_x: 238.7647,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(568),
            Item::Rect(Rect {
                max_x: 238.7647,
                max_y: 22.826088,
                min_x: 232.94116,
                min_y: 20.543478,
            }),
        ),
        (
            ItemId(569),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 22.826088,
                min_x: 229.0588,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(570),
            Item::Rect(Rect {
                max_x: 229.0588,
                max_y: 18.26087,
                min_x: 227.11765,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(571),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 9.130435,
                min_x: 227.11765,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(572),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 9.130435,
                min_x: 227.11765,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(573),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 9.130435,
                min_x: 227.11765,
                min_y: 8.400001,
            }),
        ),
        (
            ItemId(574),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 7.9523144,
                min_x: 0.0,
                min_y: 7.826087,
            }),
        ),
        (
            ItemId(575),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 9.130435,
                min_x: 273.70587,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(576),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 9.130435,
                min_x: 279.52942,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(577),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 9.130435,
                min_x: 279.52942,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(578),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 18.26087,
                min_x: 274.67648,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(579),
            Item::Rect(Rect {
                max_x: 276.61765,
                max_y: 27.391304,
                min_x: 274.67648,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(580),
            Item::Rect(Rect {
                max_x: 276.61765,
                max_y: 28.369566,
                min_x: 273.70587,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(581),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 28.369566,
                min_x: 273.03394,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(582),
            Item::Rect(Rect {
                max_x: 273.4902,
                max_y: 27.391304,
                min_x: 273.03394,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(583),
            Item::Rect(Rect {
                max_x: 273.4902,
                max_y: 18.26087,
                min_x: 267.88235,
                min_y: 10.347826,
            }),
        ),
        (
            ItemId(584),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 10.347826,
                min_x: 267.13092,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(585),
            Item::Rect(Rect {
                max_x: 267.13092,
                max_y: 9.130435,
                min_x: 262.0588,
                min_y: 0.62968516,
            }),
        ),
        (
            ItemId(586),
            Item::Rect(Rect {
                max_x: 180.5294,
                max_y: 178.04349,
                min_x: 174.70587,
                min_y: 178.04349,
            }),
        ),
        (
            ItemId(587),
            Item::Rect(Rect {
                max_x: 186.35294,
                max_y: 182.6087,
                min_x: 180.5294,
                min_y: 178.04349,
            }),
        ),
        (
            ItemId(588),
            Item::Rect(Rect {
                max_x: 186.35294,
                max_y: 182.6087,
                min_x: 186.35294,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(589),
            Item::Rect(Rect {
                max_x: 192.17647,
                max_y: 191.73914,
                min_x: 186.35294,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(590),
            Item::Rect(Rect {
                max_x: 192.17647,
                max_y: 191.73914,
                min_x: 192.17647,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(591),
            Item::Rect(Rect {
                max_x: 198.0,
                max_y: 196.30435,
                min_x: 192.17647,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(592),
            Item::Rect(Rect {
                max_x: 203.82352,
                max_y: 196.30435,
                min_x: 198.0,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(593),
            Item::Rect(Rect {
                max_x: 203.82352,
                max_y: 191.73914,
                min_x: 198.0,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(594),
            Item::Rect(Rect {
                max_x: 198.0,
                max_y: 182.6087,
                min_x: 198.0,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(595),
            Item::Rect(Rect {
                max_x: 198.0,
                max_y: 182.6087,
                min_x: 198.0,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(596),
            Item::Rect(Rect {
                max_x: 203.82352,
                max_y: 182.6087,
                min_x: 198.0,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(597),
            Item::Rect(Rect {
                max_x: 209.64705,
                max_y: 182.6087,
                min_x: 203.82352,
                min_y: 178.04349,
            }),
        ),
        (
            ItemId(598),
            Item::Rect(Rect {
                max_x: 215.47058,
                max_y: 182.6087,
                min_x: 209.64705,
                min_y: 178.04349,
            }),
        ),
        (
            ItemId(599),
            Item::Rect(Rect {
                max_x: 215.47058,
                max_y: 182.6087,
                min_x: 215.47058,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(600),
            Item::Rect(Rect {
                max_x: 215.47058,
                max_y: 182.6087,
                min_x: 215.47058,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(601),
            Item::Rect(Rect {
                max_x: 215.47058,
                max_y: 191.73914,
                min_x: 209.64705,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(602),
            Item::Rect(Rect {
                max_x: 209.64705,
                max_y: 200.86957,
                min_x: 209.64705,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(603),
            Item::Rect(Rect {
                max_x: 180.5294,
                max_y: 118.695656,
                min_x: 180.5294,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(604),
            Item::Rect(Rect {
                max_x: 180.5294,
                max_y: 118.695656,
                min_x: 180.5294,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(605),
            Item::Rect(Rect {
                max_x: 180.5294,
                max_y: 118.695656,
                min_x: 180.5294,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(606),
            Item::Rect(Rect {
                max_x: 180.5294,
                max_y: 118.695656,
                min_x: 180.5294,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(607),
            Item::Rect(Rect {
                max_x: 101.91176,
                max_y: 191.73914,
                min_x: 99.0,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(608),
            Item::Rect(Rect {
                max_x: 101.91176,
                max_y: 182.6087,
                min_x: 99.0,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(609),
            Item::Rect(Rect {
                max_x: 101.91176,
                max_y: 173.47827,
                min_x: 99.0,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(610),
            Item::Rect(Rect {
                max_x: 104.823524,
                max_y: 164.34782,
                min_x: 101.91176,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(611),
            Item::Rect(Rect {
                max_x: 104.823524,
                max_y: 155.21739,
                min_x: 104.823524,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(612),
            Item::Rect(Rect {
                max_x: 104.823524,
                max_y: 146.08696,
                min_x: 99.0,
                min_y: 143.04349,
            }),
        ),
        (
            ItemId(613),
            Item::Rect(Rect {
                max_x: 99.0,
                max_y: 143.04349,
                min_x: 96.088234,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(614),
            Item::Rect(Rect {
                max_x: 96.088234,
                max_y: 136.95653,
                min_x: 93.17647,
                min_y: 130.86957,
            }),
        ),
        (
            ItemId(615),
            Item::Rect(Rect {
                max_x: 93.17647,
                max_y: 130.86957,
                min_x: 87.352936,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(616),
            Item::Rect(Rect {
                max_x: 93.17647,
                max_y: 127.82609,
                min_x: 87.352936,
                min_y: 124.782616,
            }),
        ),
        (
            ItemId(617),
            Item::Rect(Rect {
                max_x: 99.0,
                max_y: 124.782616,
                min_x: 93.17647,
                min_y: 120.978264,
            }),
        ),
        (
            ItemId(618),
            Item::Rect(Rect {
                max_x: 100.45588,
                max_y: 120.978264,
                min_x: 99.0,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(619),
            Item::Rect(Rect {
                max_x: 100.45588,
                max_y: 118.695656,
                min_x: 99.0,
                min_y: 114.13044,
            }),
        ),
        (
            ItemId(620),
            Item::Rect(Rect {
                max_x: 99.0,
                max_y: 114.13044,
                min_x: 93.17647,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(621),
            Item::Rect(Rect {
                max_x: 93.17647,
                max_y: 109.565216,
                min_x: 93.17647,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(622),
            Item::Rect(Rect {
                max_x: 93.17647,
                max_y: 100.434784,
                min_x: 87.352936,
                min_y: 95.86957,
            }),
        ),
        (
            ItemId(623),
            Item::Rect(Rect {
                max_x: 87.352936,
                max_y: 95.86957,
                min_x: 81.52941,
                min_y: 95.86957,
            }),
        ),
        (
            ItemId(624),
            Item::Rect(Rect {
                max_x: 81.52941,
                max_y: 100.434784,
                min_x: 75.70588,
                min_y: 95.86957,
            }),
        ),
        (
            ItemId(625),
            Item::Rect(Rect {
                max_x: 75.70588,
                max_y: 100.434784,
                min_x: 69.882355,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(626),
            Item::Rect(Rect {
                max_x: 69.882355,
                max_y: 100.434784,
                min_x: 64.05882,
                min_y: 95.86957,
            }),
        ),
        (
            ItemId(627),
            Item::Rect(Rect {
                max_x: 64.05882,
                max_y: 95.86957,
                min_x: 58.23529,
                min_y: 95.86957,
            }),
        ),
        (
            ItemId(628),
            Item::Rect(Rect {
                max_x: 58.23529,
                max_y: 100.434784,
                min_x: 52.411762,
                min_y: 95.86957,
            }),
        ),
        (
            ItemId(629),
            Item::Rect(Rect {
                max_x: 54.35294,
                max_y: 109.565216,
                min_x: 52.411762,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(630),
            Item::Rect(Rect {
                max_x: 55.32353,
                max_y: 118.695656,
                min_x: 54.35294,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(631),
            Item::Rect(Rect {
                max_x: 58.23529,
                max_y: 121.73913,
                min_x: 55.32353,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(632),
            Item::Rect(Rect {
                max_x: 64.05882,
                max_y: 121.73913,
                min_x: 58.23529,
                min_y: 121.73913,
            }),
        ),
        (
            ItemId(633),
            Item::Rect(Rect {
                max_x: 69.882355,
                max_y: 121.73913,
                min_x: 64.05882,
                min_y: 121.73913,
            }),
        ),
        (
            ItemId(634),
            Item::Rect(Rect {
                max_x: 75.70588,
                max_y: 123.26087,
                min_x: 69.882355,
                min_y: 121.73913,
            }),
        ),
        (
            ItemId(635),
            Item::Rect(Rect {
                max_x: 81.52941,
                max_y: 127.82609,
                min_x: 75.70588,
                min_y: 123.26087,
            }),
        ),
        (
            ItemId(636),
            Item::Rect(Rect {
                max_x: 87.352936,
                max_y: 127.82609,
                min_x: 81.52941,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(637),
            Item::Rect(Rect {
                max_x: 87.352936,
                max_y: 136.95653,
                min_x: 81.52941,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(638),
            Item::Rect(Rect {
                max_x: 81.52941,
                max_y: 136.95653,
                min_x: 75.70588,
                min_y: 132.39131,
            }),
        ),
        (
            ItemId(639),
            Item::Rect(Rect {
                max_x: 75.70588,
                max_y: 136.95653,
                min_x: 72.79411,
                min_y: 132.39131,
            }),
        ),
        (
            ItemId(640),
            Item::Rect(Rect {
                max_x: 72.79411,
                max_y: 146.08696,
                min_x: 69.882355,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(641),
            Item::Rect(Rect {
                max_x: 69.882355,
                max_y: 155.21739,
                min_x: 66.97059,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(642),
            Item::Rect(Rect {
                max_x: 69.882355,
                max_y: 164.34782,
                min_x: 66.97059,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(643),
            Item::Rect(Rect {
                max_x: 69.882355,
                max_y: 164.34782,
                min_x: 69.882355,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(644),
            Item::Rect(Rect {
                max_x: 75.70588,
                max_y: 173.47827,
                min_x: 69.882355,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(645),
            Item::Rect(Rect {
                max_x: 81.52941,
                max_y: 173.47827,
                min_x: 75.70588,
                min_y: 168.91304,
            }),
        ),
        (
            ItemId(646),
            Item::Rect(Rect {
                max_x: 87.352936,
                max_y: 168.91304,
                min_x: 81.52941,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(647),
            Item::Rect(Rect {
                max_x: 87.352936,
                max_y: 164.34782,
                min_x: 87.352936,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(648),
            Item::Rect(Rect {
                max_x: 93.17647,
                max_y: 173.47827,
                min_x: 87.352936,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(649),
            Item::Rect(Rect {
                max_x: 99.0,
                max_y: 173.47827,
                min_x: 93.17647,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(650),
            Item::Rect(Rect {
                max_x: 99.0,
                max_y: 173.47827,
                min_x: 93.17647,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(651),
            Item::Rect(Rect {
                max_x: 93.17647,
                max_y: 178.04349,
                min_x: 87.352936,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(652),
            Item::Rect(Rect {
                max_x: 87.352936,
                max_y: 182.6087,
                min_x: 81.52941,
                min_y: 178.04349,
            }),
        ),
        (
            ItemId(653),
            Item::Rect(Rect {
                max_x: 81.52941,
                max_y: 182.6087,
                min_x: 75.70588,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(654),
            Item::Rect(Rect {
                max_x: 75.70588,
                max_y: 182.6087,
                min_x: 69.882355,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(655),
            Item::Rect(Rect {
                max_x: 75.70588,
                max_y: 191.73914,
                min_x: 69.882355,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(656),
            Item::Rect(Rect {
                max_x: 75.70588,
                max_y: 191.73914,
                min_x: 75.70588,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(657),
            Item::Rect(Rect {
                max_x: 81.52941,
                max_y: 200.86957,
                min_x: 75.70588,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(658),
            Item::Rect(Rect {
                max_x: 87.352936,
                max_y: 200.86957,
                min_x: 81.52941,
                min_y: 200.86957,
            }),
        ),
        (
            ItemId(659),
            Item::Rect(Rect {
                max_x: 93.17647,
                max_y: 200.86957,
                min_x: 87.352936,
                min_y: 200.86957,
            }),
        ),
        (
            ItemId(660),
            Item::Rect(Rect {
                max_x: 99.0,
                max_y: 200.86957,
                min_x: 93.17647,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(661),
            Item::Rect(Rect {
                max_x: 99.0,
                max_y: 191.73914,
                min_x: 99.0,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(662),
            Item::Rect(Rect {
                max_x: 285.81882,
                max_y: 109.565216,
                min_x: 285.35294,
                min_y: 108.88889,
            }),
        ),
        (
            ItemId(663),
            Item::Rect(Rect {
                max_x: 285.81882,
                max_y: 115.652176,
                min_x: 285.35294,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(664),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 115.652176,
                min_x: 283.0235,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(665),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 109.565216,
                min_x: 283.0235,
                min_y: 108.88889,
            }),
        ),
        (
            ItemId(666),
            Item::Rect(Rect {
                max_x: 17.470589,
                max_y: 182.6087,
                min_x: 17.470589,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(667),
            Item::Rect(Rect {
                max_x: 17.470589,
                max_y: 182.6087,
                min_x: 17.470589,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(668),
            Item::Rect(Rect {
                max_x: 23.294117,
                max_y: 182.6087,
                min_x: 17.470589,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(669),
            Item::Rect(Rect {
                max_x: 23.294117,
                max_y: 173.47827,
                min_x: 23.294117,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(670),
            Item::Rect(Rect {
                max_x: 26.205881,
                max_y: 173.47827,
                min_x: 23.294117,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(671),
            Item::Rect(Rect {
                max_x: 29.117645,
                max_y: 164.34782,
                min_x: 26.205881,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(672),
            Item::Rect(Rect {
                max_x: 29.117645,
                max_y: 155.21739,
                min_x: 23.294117,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(673),
            Item::Rect(Rect {
                max_x: 23.294117,
                max_y: 155.21739,
                min_x: 17.470589,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(674),
            Item::Rect(Rect {
                max_x: 17.470589,
                max_y: 155.21739,
                min_x: 11.6470585,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(675),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 157.5,
                min_x: 5.8235292,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(676),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 157.5,
                min_x: 2.9117646,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(677),
            Item::Rect(Rect {
                max_x: 157.23529,
                max_y: 178.04349,
                min_x: 151.41176,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(678),
            Item::Rect(Rect {
                max_x: 160.14705,
                max_y: 182.6087,
                min_x: 157.23529,
                min_y: 178.04349,
            }),
        ),
        (
            ItemId(679),
            Item::Rect(Rect {
                max_x: 163.05882,
                max_y: 187.17392,
                min_x: 160.14705,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(680),
            Item::Rect(Rect {
                max_x: 168.88235,
                max_y: 187.17392,
                min_x: 163.05882,
                min_y: 187.17392,
            }),
        ),
        (
            ItemId(681),
            Item::Rect(Rect {
                max_x: 171.79411,
                max_y: 187.17392,
                min_x: 168.88235,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(682),
            Item::Rect(Rect {
                max_x: 174.70587,
                max_y: 182.6087,
                min_x: 171.79411,
                min_y: 178.04349,
            }),
        ),
        (
            ItemId(683),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 45.652176,
                min_x: 285.35294,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(684),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 45.652176,
                min_x: 285.35294,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(685),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 45.652176,
                min_x: 285.35294,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(686),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 45.652176,
                min_x: 285.35294,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(687),
            Item::Rect(Rect {
                max_x: 122.29411,
                max_y: 50.217392,
                min_x: 116.47058,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(688),
            Item::Rect(Rect {
                max_x: 122.29411,
                max_y: 45.652176,
                min_x: 122.29411,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(689),
            Item::Rect(Rect {
                max_x: 128.11765,
                max_y: 45.652176,
                min_x: 122.29411,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(690),
            Item::Rect(Rect {
                max_x: 128.11765,
                max_y: 45.652176,
                min_x: 128.11765,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(691),
            Item::Rect(Rect {
                max_x: 133.94118,
                max_y: 50.217392,
                min_x: 128.11765,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(692),
            Item::Rect(Rect {
                max_x: 139.76471,
                max_y: 51.739132,
                min_x: 133.94118,
                min_y: 50.217392,
            }),
        ),
        (
            ItemId(693),
            Item::Rect(Rect {
                max_x: 142.67647,
                max_y: 54.782608,
                min_x: 139.76471,
                min_y: 51.739132,
            }),
        ),
        (
            ItemId(694),
            Item::Rect(Rect {
                max_x: 142.67647,
                max_y: 63.913044,
                min_x: 139.76471,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(695),
            Item::Rect(Rect {
                max_x: 139.76471,
                max_y: 73.04348,
                min_x: 133.94118,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(696),
            Item::Rect(Rect {
                max_x: 133.94118,
                max_y: 73.04348,
                min_x: 128.11765,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(697),
            Item::Rect(Rect {
                max_x: 128.11765,
                max_y: 63.913044,
                min_x: 122.29411,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(698),
            Item::Rect(Rect {
                max_x: 122.29411,
                max_y: 63.913044,
                min_x: 116.47058,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(699),
            Item::Rect(Rect {
                max_x: 116.47058,
                max_y: 63.913044,
                min_x: 110.64706,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(700),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 54.782608,
                min_x: 110.64706,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(701),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 45.652176,
                min_x: 110.64706,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(702),
            Item::Rect(Rect {
                max_x: 116.47058,
                max_y: 50.217392,
                min_x: 110.64706,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(703),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 200.86957,
                min_x: 267.88235,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(704),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 191.73914,
                min_x: 267.88235,
                min_y: 187.17392,
            }),
        ),
        (
            ItemId(705),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 188.69565,
                min_x: 262.0588,
                min_y: 187.17392,
            }),
        ),
        (
            ItemId(706),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 188.69565,
                min_x: 256.2353,
                min_y: 187.17392,
            }),
        ),
        (
            ItemId(707),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 188.69565,
                min_x: 250.41176,
                min_y: 187.17392,
            }),
        ),
        (
            ItemId(708),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 191.73914,
                min_x: 244.58823,
                min_y: 188.69565,
            }),
        ),
        (
            ItemId(709),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 191.73914,
                min_x: 238.7647,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(710),
            Item::Rect(Rect {
                max_x: 238.7647,
                max_y: 191.73914,
                min_x: 232.94116,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(711),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 191.73914,
                min_x: 232.94116,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(712),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 182.6087,
                min_x: 232.94116,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(713),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 173.47827,
                min_x: 227.11765,
                min_y: 170.43478,
            }),
        ),
        (
            ItemId(714),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 170.43478,
                min_x: 221.29411,
                min_y: 168.91304,
            }),
        ),
        (
            ItemId(715),
            Item::Rect(Rect {
                max_x: 221.29411,
                max_y: 168.91304,
                min_x: 218.38234,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(716),
            Item::Rect(Rect {
                max_x: 218.38234,
                max_y: 164.34782,
                min_x: 215.47058,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(717),
            Item::Rect(Rect {
                max_x: 215.47058,
                max_y: 155.21739,
                min_x: 209.64705,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(718),
            Item::Rect(Rect {
                max_x: 209.64705,
                max_y: 155.21739,
                min_x: 203.82352,
                min_y: 150.65218,
            }),
        ),
        (
            ItemId(719),
            Item::Rect(Rect {
                max_x: 203.82352,
                max_y: 150.65218,
                min_x: 198.0,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(720),
            Item::Rect(Rect {
                max_x: 198.0,
                max_y: 149.13045,
                min_x: 192.17647,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(721),
            Item::Rect(Rect {
                max_x: 192.17647,
                max_y: 149.13045,
                min_x: 186.35294,
                min_y: 149.13045,
            }),
        ),
        (
            ItemId(722),
            Item::Rect(Rect {
                max_x: 186.35294,
                max_y: 155.21739,
                min_x: 180.5294,
                min_y: 149.13045,
            }),
        ),
        (
            ItemId(723),
            Item::Rect(Rect {
                max_x: 183.44118,
                max_y: 164.34782,
                min_x: 180.5294,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(724),
            Item::Rect(Rect {
                max_x: 183.44118,
                max_y: 168.91304,
                min_x: 180.5294,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(725),
            Item::Rect(Rect {
                max_x: 180.5294,
                max_y: 168.91304,
                min_x: 174.70587,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(726),
            Item::Rect(Rect {
                max_x: 174.70587,
                max_y: 164.34782,
                min_x: 174.70587,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(727),
            Item::Rect(Rect {
                max_x: 174.70587,
                max_y: 155.21739,
                min_x: 174.70587,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(728),
            Item::Rect(Rect {
                max_x: 180.5294,
                max_y: 155.21739,
                min_x: 174.70587,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(729),
            Item::Rect(Rect {
                max_x: 180.5294,
                max_y: 146.08696,
                min_x: 174.70587,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(730),
            Item::Rect(Rect {
                max_x: 174.70587,
                max_y: 146.08696,
                min_x: 172.7647,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(731),
            Item::Rect(Rect {
                max_x: 172.7647,
                max_y: 152.1739,
                min_x: 168.88235,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(732),
            Item::Rect(Rect {
                max_x: 168.88235,
                max_y: 152.1739,
                min_x: 163.05882,
                min_y: 150.65218,
            }),
        ),
        (
            ItemId(733),
            Item::Rect(Rect {
                max_x: 163.05882,
                max_y: 155.21739,
                min_x: 157.23529,
                min_y: 150.65218,
            }),
        ),
        (
            ItemId(734),
            Item::Rect(Rect {
                max_x: 157.23529,
                max_y: 164.34782,
                min_x: 151.41176,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(735),
            Item::Rect(Rect {
                max_x: 151.41176,
                max_y: 173.47827,
                min_x: 151.41176,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(736),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 200.86957,
                min_x: 267.88235,
                min_y: 200.86957,
            }),
        ),
        (
            ItemId(737),
            Item::Rect(Rect {
                max_x: 209.64705,
                max_y: 9.130435,
                min_x: 209.64705,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(738),
            Item::Rect(Rect {
                max_x: 69.882355,
                max_y: 100.434784,
                min_x: 69.882355,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(739),
            Item::Rect(Rect {
                max_x: 139.76471,
                max_y: 136.95653,
                min_x: 133.94118,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(740),
            Item::Rect(Rect {
                max_x: 139.76471,
                max_y: 146.08696,
                min_x: 133.94118,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(741),
            Item::Rect(Rect {
                max_x: 145.58823,
                max_y: 146.08696,
                min_x: 139.76471,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(742),
            Item::Rect(Rect {
                max_x: 145.58823,
                max_y: 136.95653,
                min_x: 139.76471,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(743),
            Item::Rect(Rect {
                max_x: 174.70587,
                max_y: 9.130435,
                min_x: 174.70587,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(744),
            Item::Rect(Rect {
                max_x: 171.79411,
                max_y: 39.56522,
                min_x: 168.88235,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(745),
            Item::Rect(Rect {
                max_x: 168.88235,
                max_y: 45.652176,
                min_x: 163.05882,
                min_y: 39.56522,
            }),
        ),
        (
            ItemId(746),
            Item::Rect(Rect {
                max_x: 163.05882,
                max_y: 45.652176,
                min_x: 157.23529,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(747),
            Item::Rect(Rect {
                max_x: 157.23529,
                max_y: 45.652176,
                min_x: 151.41176,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(748),
            Item::Rect(Rect {
                max_x: 151.41176,
                max_y: 45.652176,
                min_x: 145.58823,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(749),
            Item::Rect(Rect {
                max_x: 145.58823,
                max_y: 36.52174,
                min_x: 139.76471,
                min_y: 31.956522,
            }),
        ),
        (
            ItemId(750),
            Item::Rect(Rect {
                max_x: 139.76471,
                max_y: 31.956522,
                min_x: 133.94118,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(751),
            Item::Rect(Rect {
                max_x: 133.94118,
                max_y: 27.391304,
                min_x: 133.94118,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(752),
            Item::Rect(Rect {
                max_x: 133.94118,
                max_y: 18.26087,
                min_x: 133.94118,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(753),
            Item::Rect(Rect {
                max_x: 133.94118,
                max_y: 9.130435,
                min_x: 133.94118,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(754),
            Item::Rect(Rect {
                max_x: 133.94118,
                max_y: 18.26087,
                min_x: 133.94118,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(755),
            Item::Rect(Rect {
                max_x: 139.76471,
                max_y: 24.347828,
                min_x: 133.94118,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(756),
            Item::Rect(Rect {
                max_x: 145.58823,
                max_y: 27.391304,
                min_x: 139.76471,
                min_y: 24.347828,
            }),
        ),
        (
            ItemId(757),
            Item::Rect(Rect {
                max_x: 145.58823,
                max_y: 27.391304,
                min_x: 145.58823,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(758),
            Item::Rect(Rect {
                max_x: 151.41176,
                max_y: 31.956522,
                min_x: 145.58823,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(759),
            Item::Rect(Rect {
                max_x: 157.23529,
                max_y: 31.956522,
                min_x: 151.41176,
                min_y: 30.434782,
            }),
        ),
        (
            ItemId(760),
            Item::Rect(Rect {
                max_x: 163.05882,
                max_y: 30.434782,
                min_x: 157.23529,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(761),
            Item::Rect(Rect {
                max_x: 163.05882,
                max_y: 27.391304,
                min_x: 163.05882,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(762),
            Item::Rect(Rect {
                max_x: 168.88235,
                max_y: 27.391304,
                min_x: 163.05882,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(763),
            Item::Rect(Rect {
                max_x: 168.88235,
                max_y: 27.391304,
                min_x: 168.88235,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(764),
            Item::Rect(Rect {
                max_x: 171.79411,
                max_y: 36.52174,
                min_x: 168.88235,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(765),
            Item::Rect(Rect {
                max_x: 115.88823,
                max_y: 117.51753,
                min_x: 110.64706,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(766),
            Item::Rect(Rect {
                max_x: 115.88823,
                max_y: 109.565216,
                min_x: 110.64706,
                min_y: 101.064476,
            }),
        ),
        (
            ItemId(767),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 109.565216,
                min_x: 105.57495,
                min_y: 101.064476,
            }),
        ),
        (
            ItemId(768),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 117.51753,
                min_x: 105.57495,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(769),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 155.21739,
                min_x: 256.2353,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(770),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 155.21739,
                min_x: 256.2353,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(771),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 155.21739,
                min_x: 250.41176,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(772),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 155.21739,
                min_x: 250.41176,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(773),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 155.21739,
                min_x: 250.41176,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(774),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 155.21739,
                min_x: 250.41176,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(775),
            Item::Rect(Rect {
                max_x: 281.85883,
                max_y: 37.19807,
                min_x: 279.52942,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(776),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 37.19807,
                min_x: 279.09802,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(777),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 36.52174,
                min_x: 279.09802,
                min_y: 32.869564,
            }),
        ),
        (
            ItemId(778),
            Item::Rect(Rect {
                max_x: 281.85883,
                max_y: 36.52174,
                min_x: 279.52942,
                min_y: 32.869564,
            }),
        ),
        (
            ItemId(779),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 100.434784,
                min_x: 244.58823,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(780),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 100.434784,
                min_x: 250.41176,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(781),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 100.434784,
                min_x: 250.41176,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(782),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 109.565216,
                min_x: 244.58823,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(783),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 109.565216,
                min_x: 244.58823,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(784),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 100.434784,
                min_x: 244.58823,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(785),
            Item::Rect(Rect {
                max_x: 221.29411,
                max_y: 54.782608,
                min_x: 221.29411,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(786),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 100.434784,
                min_x: 232.94116,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(787),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 100.434784,
                min_x: 227.11765,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(788),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 100.434784,
                min_x: 227.11765,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(789),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 91.30435,
                min_x: 227.11765,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(790),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 82.17391,
                min_x: 227.11765,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(791),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 82.17391,
                min_x: 227.11765,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(792),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 82.17391,
                min_x: 227.11765,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(793),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 82.17391,
                min_x: 232.94116,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(794),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 91.30435,
                min_x: 232.94116,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(795),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 100.434784,
                min_x: 232.94116,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(796),
            Item::Rect(Rect {
                max_x: 29.117645,
                max_y: 182.6087,
                min_x: 29.117645,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(797),
            Item::Rect(Rect {
                max_x: 29.117645,
                max_y: 182.6087,
                min_x: 23.294117,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(798),
            Item::Rect(Rect {
                max_x: 23.294117,
                max_y: 182.6087,
                min_x: 17.470589,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(799),
            Item::Rect(Rect {
                max_x: 40.764706,
                max_y: 173.47827,
                min_x: 34.941177,
                min_y: 168.91304,
            }),
        ),
        (
            ItemId(800),
            Item::Rect(Rect {
                max_x: 34.941177,
                max_y: 173.47827,
                min_x: 29.117645,
                min_y: 168.91304,
            }),
        ),
        (
            ItemId(801),
            Item::Rect(Rect {
                max_x: 29.117645,
                max_y: 182.6087,
                min_x: 29.117645,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(802),
            Item::Rect(Rect {
                max_x: 268.5543,
                max_y: 63.913044,
                min_x: 267.88235,
                min_y: 62.859535,
            }),
        ),
        (
            ItemId(803),
            Item::Rect(Rect {
                max_x: 268.5543,
                max_y: 73.04348,
                min_x: 267.88235,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(804),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 73.04348,
                min_x: 264.38824,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(805),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 63.913044,
                min_x: 264.38824,
                min_y: 62.859535,
            }),
        ),
        (
            ItemId(806),
            Item::Rect(Rect {
                max_x: 29.117645,
                max_y: 200.86957,
                min_x: 23.294117,
                min_y: 200.86957,
            }),
        ),
        (
            ItemId(807),
            Item::Rect(Rect {
                max_x: 34.941177,
                max_y: 200.86957,
                min_x: 29.117645,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(808),
            Item::Rect(Rect {
                max_x: 34.941177,
                max_y: 191.73914,
                min_x: 34.941177,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(809),
            Item::Rect(Rect {
                max_x: 40.764706,
                max_y: 200.86957,
                min_x: 34.941177,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(810),
            Item::Rect(Rect {
                max_x: 40.764706,
                max_y: 200.86957,
                min_x: 40.764706,
                min_y: 200.86957,
            }),
        ),
        (
            ItemId(811),
            Item::Rect(Rect {
                max_x: 17.470589,
                max_y: 121.73913,
                min_x: 11.6470585,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(812),
            Item::Rect(Rect {
                max_x: 23.294117,
                max_y: 121.73913,
                min_x: 17.470589,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(813),
            Item::Rect(Rect {
                max_x: 23.294117,
                max_y: 118.695656,
                min_x: 17.470589,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(814),
            Item::Rect(Rect {
                max_x: 17.470589,
                max_y: 118.695656,
                min_x: 11.6470585,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(815),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 54.782608,
                min_x: 11.6470585,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(816),
            Item::Rect(Rect {
                max_x: 186.35294,
                max_y: 100.434784,
                min_x: 186.35294,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(817),
            Item::Rect(Rect {
                max_x: 186.35294,
                max_y: 100.434784,
                min_x: 186.35294,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(818),
            Item::Rect(Rect {
                max_x: 186.35294,
                max_y: 91.30435,
                min_x: 186.35294,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(819),
            Item::Rect(Rect {
                max_x: 186.35294,
                max_y: 91.30435,
                min_x: 186.35294,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(820),
            Item::Rect(Rect {
                max_x: 139.76471,
                max_y: 63.913044,
                min_x: 139.76471,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(821),
            Item::Rect(Rect {
                max_x: 203.82352,
                max_y: 109.565216,
                min_x: 203.82352,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(822),
            Item::Rect(Rect {
                max_x: 174.70587,
                max_y: 164.34782,
                min_x: 174.70587,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(823),
            Item::Rect(Rect {
                max_x: 203.82352,
                max_y: 73.04348,
                min_x: 203.82352,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(824),
            Item::Rect(Rect {
                max_x: 46.588234,
                max_y: 159.78261,
                min_x: 40.764706,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(825),
            Item::Rect(Rect {
                max_x: 49.5,
                max_y: 159.78261,
                min_x: 46.588234,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(826),
            Item::Rect(Rect {
                max_x: 49.5,
                max_y: 155.21739,
                min_x: 49.5,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(827),
            Item::Rect(Rect {
                max_x: 49.5,
                max_y: 146.08696,
                min_x: 46.588234,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(828),
            Item::Rect(Rect {
                max_x: 46.588234,
                max_y: 146.08696,
                min_x: 40.764706,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(829),
            Item::Rect(Rect {
                max_x: 40.764706,
                max_y: 155.21739,
                min_x: 40.764706,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(830),
            Item::Rect(Rect {
                max_x: 93.17647,
                max_y: 109.565216,
                min_x: 93.17647,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(831),
            Item::Rect(Rect {
                max_x: 81.52941,
                max_y: 54.782608,
                min_x: 78.617645,
                min_y: 51.739132,
            }),
        ),
        (
            ItemId(832),
            Item::Rect(Rect {
                max_x: 81.52941,
                max_y: 63.913044,
                min_x: 78.617645,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(833),
            Item::Rect(Rect {
                max_x: 84.44118,
                max_y: 63.913044,
                min_x: 81.52941,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(834),
            Item::Rect(Rect {
                max_x: 84.44118,
                max_y: 54.782608,
                min_x: 81.52941,
                min_y: 51.739132,
            }),
        ),
        (
            ItemId(835),
            Item::Rect(Rect {
                max_x: 221.29411,
                max_y: 100.434784,
                min_x: 221.29411,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(836),
            Item::Rect(Rect {
                max_x: 221.29411,
                max_y: 100.434784,
                min_x: 221.29411,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(837),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 100.434784,
                min_x: 221.29411,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(838),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 100.434784,
                min_x: 221.29411,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(839),
            Item::Rect(Rect {
                max_x: 186.35294,
                max_y: 100.434784,
                min_x: 186.35294,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(840),
            Item::Rect(Rect {
                max_x: 186.35294,
                max_y: 100.434784,
                min_x: 186.35294,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(841),
            Item::Rect(Rect {
                max_x: 133.94118,
                max_y: 9.130435,
                min_x: 133.94118,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(842),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 73.04348,
                min_x: 267.88235,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(843),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 73.04348,
                min_x: 267.88235,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(844),
            Item::Rect(Rect {
                max_x: 145.58823,
                max_y: 36.52174,
                min_x: 145.58823,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(845),
            Item::Rect(Rect {
                max_x: 163.05882,
                max_y: 91.30435,
                min_x: 163.05882,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(846),
            Item::Rect(Rect {
                max_x: 163.05882,
                max_y: 91.30435,
                min_x: 163.05882,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(847),
            Item::Rect(Rect {
                max_x: 168.88235,
                max_y: 91.30435,
                min_x: 163.05882,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(848),
            Item::Rect(Rect {
                max_x: 168.88235,
                max_y: 82.17391,
                min_x: 168.88235,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(849),
            Item::Rect(Rect {
                max_x: 168.88235,
                max_y: 91.30435,
                min_x: 168.88235,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(850),
            Item::Rect(Rect {
                max_x: 168.88235,
                max_y: 91.30435,
                min_x: 163.05882,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(851),
            Item::Rect(Rect {
                max_x: 122.29411,
                max_y: 155.21739,
                min_x: 116.47058,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(852),
            Item::Rect(Rect {
                max_x: 122.29411,
                max_y: 164.34782,
                min_x: 116.47058,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(853),
            Item::Rect(Rect {
                max_x: 128.11765,
                max_y: 164.34782,
                min_x: 122.29411,
                min_y: 161.30435,
            }),
        ),
        (
            ItemId(854),
            Item::Rect(Rect {
                max_x: 133.94118,
                max_y: 161.30435,
                min_x: 128.11765,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(855),
            Item::Rect(Rect {
                max_x: 133.94118,
                max_y: 155.21739,
                min_x: 128.11765,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(856),
            Item::Rect(Rect {
                max_x: 128.11765,
                max_y: 146.08696,
                min_x: 122.29411,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(857),
            Item::Rect(Rect {
                max_x: 151.41176,
                max_y: 45.652176,
                min_x: 151.41176,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(858),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 82.17391,
                min_x: 275.64703,
                min_y: 80.82125,
            }),
        ),
        (
            ItemId(859),
            Item::Rect(Rect {
                max_x: 280.39215,
                max_y: 82.17391,
                min_x: 279.52942,
                min_y: 80.82125,
            }),
        ),
        (
            ItemId(860),
            Item::Rect(Rect {
                max_x: 280.39215,
                max_y: 91.30435,
                min_x: 279.76236,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(861),
            Item::Rect(Rect {
                max_x: 279.76236,
                max_y: 94.347824,
                min_x: 279.52942,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(862),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 94.347824,
                min_x: 278.07352,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(863),
            Item::Rect(Rect {
                max_x: 278.07352,
                max_y: 91.30435,
                min_x: 275.64703,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(864),
            Item::Rect(Rect {
                max_x: 116.47058,
                max_y: 191.73914,
                min_x: 113.55882,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(865),
            Item::Rect(Rect {
                max_x: 122.29411,
                max_y: 191.73914,
                min_x: 116.47058,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(866),
            Item::Rect(Rect {
                max_x: 125.20588,
                max_y: 191.73914,
                min_x: 122.29411,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(867),
            Item::Rect(Rect {
                max_x: 125.20588,
                max_y: 182.6087,
                min_x: 122.29411,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(868),
            Item::Rect(Rect {
                max_x: 122.29411,
                max_y: 178.04349,
                min_x: 116.47058,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(869),
            Item::Rect(Rect {
                max_x: 116.47058,
                max_y: 182.6087,
                min_x: 113.55882,
                min_y: 178.04349,
            }),
        ),
        (
            ItemId(870),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 18.26087,
                min_x: 290.20587,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(871),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 9.130435,
                min_x: 291.17645,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(872),
            Item::Rect(Rect {
                max_x: 69.882355,
                max_y: 146.08696,
                min_x: 69.882355,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(873),
            Item::Rect(Rect {
                max_x: 209.64705,
                max_y: 155.21739,
                min_x: 209.64705,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(874),
            Item::Rect(Rect {
                max_x: 221.29411,
                max_y: 141.52174,
                min_x: 215.47058,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(875),
            Item::Rect(Rect {
                max_x: 221.29411,
                max_y: 136.95653,
                min_x: 215.47058,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(876),
            Item::Rect(Rect {
                max_x: 215.47058,
                max_y: 127.82609,
                min_x: 209.64705,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(877),
            Item::Rect(Rect {
                max_x: 209.64705,
                max_y: 136.95653,
                min_x: 203.82352,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(878),
            Item::Rect(Rect {
                max_x: 209.64705,
                max_y: 141.52174,
                min_x: 203.82352,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(879),
            Item::Rect(Rect {
                max_x: 215.47058,
                max_y: 141.52174,
                min_x: 209.64705,
                min_y: 141.52174,
            }),
        ),
        (
            ItemId(880),
            Item::Rect(Rect {
                max_x: 174.70587,
                max_y: 100.434784,
                min_x: 174.70587,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(881),
            Item::Rect(Rect {
                max_x: 174.70587,
                max_y: 100.434784,
                min_x: 174.70587,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(882),
            Item::Rect(Rect {
                max_x: 174.70587,
                max_y: 100.434784,
                min_x: 174.70587,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(883),
            Item::Rect(Rect {
                max_x: 174.70587,
                max_y: 100.434784,
                min_x: 174.70587,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(884),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 191.73914,
                min_x: 232.94116,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(885),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 191.73914,
                min_x: 232.94116,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(886),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 73.04348,
                min_x: 110.64706,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(887),
            Item::Rect(Rect {
                max_x: 163.05882,
                max_y: 45.652176,
                min_x: 163.05882,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(888),
            Item::Rect(Rect {
                max_x: 145.58823,
                max_y: 9.130435,
                min_x: 145.58823,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(889),
            Item::Rect(Rect {
                max_x: 157.23529,
                max_y: 200.86957,
                min_x: 151.41176,
                min_y: 196.30435,
            }),
        ),
        (
            ItemId(890),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 109.565216,
                min_x: 244.58823,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(891),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 109.565216,
                min_x: 244.58823,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(892),
            Item::Rect(Rect {
                max_x: 186.35294,
                max_y: 18.26087,
                min_x: 186.35294,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(893),
            Item::Rect(Rect {
                max_x: 34.941177,
                max_y: 82.17391,
                min_x: 34.941177,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(894),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 164.34782,
                min_x: 273.70587,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(895),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 164.34782,
                min_x: 273.70587,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(896),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 164.34782,
                min_x: 273.70587,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(897),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 164.34782,
                min_x: 273.70587,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(898),
            Item::Rect(Rect {
                max_x: 17.470589,
                max_y: 73.04348,
                min_x: 17.470589,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(899),
            Item::Rect(Rect {
                max_x: 17.470589,
                max_y: 73.04348,
                min_x: 17.470589,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(900),
            Item::Rect(Rect {
                max_x: 17.470589,
                max_y: 73.04348,
                min_x: 17.470589,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(901),
            Item::Rect(Rect {
                max_x: 17.470589,
                max_y: 73.04348,
                min_x: 17.470589,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(902),
            Item::Rect(Rect {
                max_x: 99.0,
                max_y: 45.652176,
                min_x: 99.0,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(903),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 45.652176,
                min_x: 110.64706,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(904),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 54.782608,
                min_x: 110.64706,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(905),
            Item::Rect(Rect {
                max_x: 168.88235,
                max_y: 82.17391,
                min_x: 168.88235,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(906),
            Item::Rect(Rect {
                max_x: 40.764706,
                max_y: 182.6087,
                min_x: 40.764706,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(907),
            Item::Rect(Rect {
                max_x: 40.764706,
                max_y: 182.6087,
                min_x: 40.764706,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(908),
            Item::Rect(Rect {
                max_x: 34.941177,
                max_y: 191.73914,
                min_x: 34.941177,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(909),
            Item::Rect(Rect {
                max_x: 174.70587,
                max_y: 200.86957,
                min_x: 170.82353,
                min_y: 194.78262,
            }),
        ),
        (
            ItemId(910),
            Item::Rect(Rect {
                max_x: 168.88235,
                max_y: 91.30435,
                min_x: 168.88235,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(911),
            Item::Rect(Rect {
                max_x: 87.352936,
                max_y: 164.34782,
                min_x: 87.352936,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(912),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 54.782608,
                min_x: 250.41176,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(913),
            Item::Rect(Rect {
                max_x: 81.52941,
                max_y: 182.6087,
                min_x: 81.52941,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(914),
            Item::Rect(Rect {
                max_x: 81.52941,
                max_y: 182.6087,
                min_x: 81.52941,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(915),
            Item::Rect(Rect {
                max_x: 46.588234,
                max_y: 191.73914,
                min_x: 46.588234,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(916),
            Item::Rect(Rect {
                max_x: 46.588234,
                max_y: 191.73914,
                min_x: 40.764706,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(917),
            Item::Rect(Rect {
                max_x: 116.47058,
                max_y: 63.913044,
                min_x: 116.47058,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(918),
            Item::Rect(Rect {
                max_x: 128.11765,
                max_y: 82.17391,
                min_x: 128.11765,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(919),
            Item::Rect(Rect {
                max_x: 128.11765,
                max_y: 82.17391,
                min_x: 128.11765,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(920),
            Item::Rect(Rect {
                max_x: 128.11765,
                max_y: 82.17391,
                min_x: 128.11765,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(921),
            Item::Rect(Rect {
                max_x: 128.11765,
                max_y: 82.17391,
                min_x: 128.11765,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(922),
            Item::Rect(Rect {
                max_x: 81.52941,
                max_y: 136.95653,
                min_x: 81.52941,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(923),
            Item::Rect(Rect {
                max_x: 81.52941,
                max_y: 136.95653,
                min_x: 81.52941,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(924),
            Item::Rect(Rect {
                max_x: 75.70588,
                max_y: 100.434784,
                min_x: 75.70588,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(925),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 127.82609,
                min_x: 5.8235292,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(926),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 191.73914,
                min_x: 244.58823,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(927),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 191.73914,
                min_x: 244.58823,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(928),
            Item::Rect(Rect {
                max_x: 160.14705,
                max_y: 200.86957,
                min_x: 157.23529,
                min_y: 196.30435,
            }),
        ),
        (
            ItemId(929),
            Item::Rect(Rect {
                max_x: 139.76471,
                max_y: 100.434784,
                min_x: 139.76471,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(930),
            Item::Rect(Rect {
                max_x: 139.76471,
                max_y: 100.434784,
                min_x: 139.76471,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(931),
            Item::Rect(Rect {
                max_x: 133.94118,
                max_y: 27.391304,
                min_x: 133.94118,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(932),
            Item::Rect(Rect {
                max_x: 157.23529,
                max_y: 18.26087,
                min_x: 157.23529,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(933),
            Item::Rect(Rect {
                max_x: 215.47058,
                max_y: 73.04348,
                min_x: 215.47058,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(934),
            Item::Rect(Rect {
                max_x: 157.23529,
                max_y: 155.21739,
                min_x: 157.23529,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(935),
            Item::Rect(Rect {
                max_x: 133.94118,
                max_y: 73.04348,
                min_x: 133.94118,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(936),
            Item::Rect(Rect {
                max_x: 133.94118,
                max_y: 73.04348,
                min_x: 133.94118,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(937),
            Item::Rect(Rect {
                max_x: 23.294117,
                max_y: 63.913044,
                min_x: 23.294117,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(938),
            Item::Rect(Rect {
                max_x: 52.411762,
                max_y: 191.73914,
                min_x: 46.588234,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(939),
            Item::Rect(Rect {
                max_x: 23.294117,
                max_y: 200.86957,
                min_x: 17.470589,
                min_y: 200.86957,
            }),
        ),
        (
            ItemId(940),
            Item::Rect(Rect {
                max_x: 17.470589,
                max_y: 200.86957,
                min_x: 17.470589,
                min_y: 200.86957,
            }),
        ),
        (
            ItemId(941),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 82.17391,
                min_x: 262.0588,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(942),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 82.17391,
                min_x: 262.0588,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(943),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 82.17391,
                min_x: 262.0588,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(944),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 82.17391,
                min_x: 262.0588,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(945),
            Item::Rect(Rect {
                max_x: 58.23529,
                max_y: 200.86957,
                min_x: 52.411762,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(946),
            Item::Rect(Rect {
                max_x: 180.5294,
                max_y: 196.30435,
                min_x: 174.70587,
                min_y: 194.78262,
            }),
        ),
        (
            ItemId(947),
            Item::Rect(Rect {
                max_x: 17.470589,
                max_y: 155.21739,
                min_x: 17.470589,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(948),
            Item::Rect(Rect {
                max_x: 29.117645,
                max_y: 63.913044,
                min_x: 29.117645,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(949),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 27.391304,
                min_x: 291.17645,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(950),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 27.391304,
                min_x: 290.20587,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(951),
            Item::Rect(Rect {
                max_x: 186.35294,
                max_y: 63.913044,
                min_x: 186.35294,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(952),
            Item::Rect(Rect {
                max_x: 183.44118,
                max_y: 200.86957,
                min_x: 180.5294,
                min_y: 196.30435,
            }),
        ),
        (
            ItemId(953),
            Item::Rect(Rect {
                max_x: 16.305882,
                max_y: 7.304348,
                min_x: 11.6470585,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(954),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 7.363254,
                min_x: 0.0,
                min_y: 7.173913,
            }),
        ),
        (
            ItemId(955),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 7.304348,
                min_x: 5.8235292,
                min_y: 7.173913,
            }),
        ),
        (
            ItemId(956),
            Item::Rect(Rect {
                max_x: 233.80391,
                max_y: 9.130435,
                min_x: 232.94116,
                min_y: 7.669565,
            }),
        ),
        (
            ItemId(957),
            Item::Rect(Rect {
                max_x: 238.7647,
                max_y: 17.20736,
                min_x: 233.80391,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(958),
            Item::Rect(Rect {
                max_x: 239.38866,
                max_y: 18.26087,
                min_x: 238.7647,
                min_y: 17.20736,
            }),
        ),
        (
            ItemId(959),
            Item::Rect(Rect {
                max_x: 239.38866,
                max_y: 25.108696,
                min_x: 238.7647,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(960),
            Item::Rect(Rect {
                max_x: 238.7647,
                max_y: 27.391304,
                min_x: 232.94116,
                min_y: 25.108696,
            }),
        ),
        (
            ItemId(961),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 27.391304,
                min_x: 227.11765,
                min_y: 22.826088,
            }),
        ),
        (
            ItemId(962),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 22.826088,
                min_x: 225.17647,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(963),
            Item::Rect(Rect {
                max_x: 225.17647,
                max_y: 18.26087,
                min_x: 221.29411,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(964),
            Item::Rect(Rect {
                max_x: 221.29411,
                max_y: 12.173914,
                min_x: 215.47058,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(965),
            Item::Rect(Rect {
                max_x: 215.47058,
                max_y: 18.26087,
                min_x: 209.64705,
                min_y: 12.173914,
            }),
        ),
        (
            ItemId(966),
            Item::Rect(Rect {
                max_x: 215.47058,
                max_y: 24.347828,
                min_x: 209.64705,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(967),
            Item::Rect(Rect {
                max_x: 218.38234,
                max_y: 27.391304,
                min_x: 215.47058,
                min_y: 24.347828,
            }),
        ),
        (
            ItemId(968),
            Item::Rect(Rect {
                max_x: 221.29411,
                max_y: 31.956522,
                min_x: 218.38234,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(969),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 31.956522,
                min_x: 221.29411,
                min_y: 31.956522,
            }),
        ),
        (
            ItemId(970),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 31.956522,
                min_x: 227.11765,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(971),
            Item::Rect(Rect {
                max_x: 238.7647,
                max_y: 30.434782,
                min_x: 232.94116,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(972),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 30.434782,
                min_x: 238.7647,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(973),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 27.391304,
                min_x: 244.58823,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(974),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 34.632683,
                min_x: 244.58823,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(975),
            Item::Rect(Rect {
                max_x: 251.70587,
                max_y: 36.52174,
                min_x: 250.41176,
                min_y: 34.632683,
            }),
        ),
        (
            ItemId(976),
            Item::Rect(Rect {
                max_x: 251.70587,
                max_y: 45.652176,
                min_x: 251.4158,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(977),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 53.768116,
                min_x: 251.4158,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(978),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 53.768116,
                min_x: 256.2353,
                min_y: 52.753624,
            }),
        ),
        (
            ItemId(979),
            Item::Rect(Rect {
                max_x: 263.35294,
                max_y: 54.782608,
                min_x: 262.0588,
                min_y: 52.753624,
            }),
        ),
        (
            ItemId(980),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 62.157192,
                min_x: 263.35294,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(981),
            Item::Rect(Rect {
                max_x: 269.00226,
                max_y: 63.913044,
                min_x: 267.88235,
                min_y: 62.157192,
            }),
        ),
        (
            ItemId(982),
            Item::Rect(Rect {
                max_x: 269.00226,
                max_y: 73.04348,
                min_x: 268.33032,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(983),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 82.17391,
                min_x: 268.33032,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(984),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 82.17391,
                min_x: 273.70587,
                min_y: 80.14493,
            }),
        ),
        (
            ItemId(985),
            Item::Rect(Rect {
                max_x: 280.82352,
                max_y: 82.17391,
                min_x: 279.52942,
                min_y: 80.14493,
            }),
        ),
        (
            ItemId(986),
            Item::Rect(Rect {
                max_x: 280.82352,
                max_y: 91.30435,
                min_x: 280.2282,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(987),
            Item::Rect(Rect {
                max_x: 280.2282,
                max_y: 100.434784,
                min_x: 279.52942,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(988),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 100.434784,
                min_x: 273.70587,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(989),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 100.434784,
                min_x: 273.70587,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(990),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 100.434784,
                min_x: 273.70587,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(991),
            Item::Rect(Rect {
                max_x: 275.16174,
                max_y: 100.434784,
                min_x: 273.70587,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(992),
            Item::Rect(Rect {
                max_x: 275.16174,
                max_y: 91.30435,
                min_x: 273.70587,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(993),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 86.739136,
                min_x: 267.88235,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(994),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 88.26087,
                min_x: 262.0588,
                min_y: 86.739136,
            }),
        ),
        (
            ItemId(995),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 91.30435,
                min_x: 259.14706,
                min_y: 88.26087,
            }),
        ),
        (
            ItemId(996),
            Item::Rect(Rect {
                max_x: 259.14706,
                max_y: 100.434784,
                min_x: 258.17645,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(997),
            Item::Rect(Rect {
                max_x: 258.17645,
                max_y: 103.478264,
                min_x: 256.2353,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(998),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 109.565216,
                min_x: 250.41176,
                min_y: 103.478264,
            }),
        ),
        (
            ItemId(999),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 118.695656,
                min_x: 244.58823,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1000),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 127.82609,
                min_x: 240.70587,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(1001),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 136.95653,
                min_x: 240.70587,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1002),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 136.95653,
                min_x: 244.58823,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1003),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 141.52174,
                min_x: 244.58823,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1004),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 141.52174,
                min_x: 250.41176,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1005),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 136.95653,
                min_x: 256.2353,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1006),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 127.82609,
                min_x: 256.2353,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1007),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 127.82609,
                min_x: 256.2353,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1008),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 127.82609,
                min_x: 262.0588,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1009),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 136.95653,
                min_x: 262.0588,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1010),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 146.08696,
                min_x: 256.2353,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1011),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 150.65218,
                min_x: 256.2353,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1012),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 155.21739,
                min_x: 262.0588,
                min_y: 150.65218,
            }),
        ),
        (
            ItemId(1013),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 155.21739,
                min_x: 267.88235,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(1014),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 158.26088,
                min_x: 267.88235,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(1015),
            Item::Rect(Rect {
                max_x: 277.58823,
                max_y: 164.34782,
                min_x: 273.70587,
                min_y: 158.26088,
            }),
        ),
        (
            ItemId(1016),
            Item::Rect(Rect {
                max_x: 277.58823,
                max_y: 170.43478,
                min_x: 273.70587,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(1017),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 173.47827,
                min_x: 267.88235,
                min_y: 170.43478,
            }),
        ),
        (
            ItemId(1018),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 176.52174,
                min_x: 267.88235,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1019),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 182.6087,
                min_x: 273.70587,
                min_y: 176.52174,
            }),
        ),
        (
            ItemId(1020),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 182.6087,
                min_x: 279.52942,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(1021),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 187.17392,
                min_x: 279.52942,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(1022),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 187.17392,
                min_x: 285.35294,
                min_y: 187.17392,
            }),
        ),
        (
            ItemId(1023),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 109.565216,
                min_x: 0.0,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(1024),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 100.434784,
                min_x: 5.8235292,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(1025),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 100.434784,
                min_x: 5.8235292,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(1026),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 91.30435,
                min_x: 11.6470585,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(1027),
            Item::Rect(Rect {
                max_x: 14.558823,
                max_y: 91.30435,
                min_x: 11.6470585,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(1028),
            Item::Rect(Rect {
                max_x: 14.558823,
                max_y: 82.17391,
                min_x: 11.6470585,
                min_y: 77.608696,
            }),
        ),
        (
            ItemId(1029),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 77.608696,
                min_x: 5.8235292,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(1030),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 73.04348,
                min_x: 0.0,
                min_y: 68.478264,
            }),
        ),
        (
            ItemId(1031),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 1.6908213,
                min_x: 256.2353,
                min_y: 1.2593703,
            }),
        ),
        (
            ItemId(1032),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 1.6908213,
                min_x: 255.11539,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1033),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 141.52174,
                min_x: 0.0,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1034),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 136.95653,
                min_x: 0.0,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1035),
            Item::Rect(Rect {
                max_x: 122.29411,
                max_y: 4.5652175,
                min_x: 116.47058,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1036),
            Item::Rect(Rect {
                max_x: 116.47058,
                max_y: 9.130435,
                min_x: 113.55882,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1037),
            Item::Rect(Rect {
                max_x: 113.55882,
                max_y: 18.26087,
                min_x: 110.64706,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1038),
            Item::Rect(Rect {
                max_x: 116.47058,
                max_y: 22.826088,
                min_x: 110.64706,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1039),
            Item::Rect(Rect {
                max_x: 122.29411,
                max_y: 22.826088,
                min_x: 116.47058,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1040),
            Item::Rect(Rect {
                max_x: 122.29411,
                max_y: 18.26087,
                min_x: 122.29411,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1041),
            Item::Rect(Rect {
                max_x: 125.20588,
                max_y: 18.26087,
                min_x: 122.29411,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1042),
            Item::Rect(Rect {
                max_x: 125.20588,
                max_y: 9.130435,
                min_x: 122.29411,
                min_y: 4.5652175,
            }),
        ),
        (
            ItemId(1043),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 8.454106,
                min_x: 227.11765,
                min_y: 7.669565,
            }),
        ),
        (
            ItemId(1044),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 7.725753,
                min_x: 285.35294,
                min_y: 6.847826,
            }),
        ),
        (
            ItemId(1045),
            Item::Rect(Rect {
                max_x: 0.0,
                max_y: 91.30435,
                min_x: 0.0,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(1046),
            Item::Rect(Rect {
                max_x: 0.0,
                max_y: 91.30435,
                min_x: 0.0,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(1047),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 191.73914,
                min_x: 0.0,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(1048),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 191.73914,
                min_x: 5.8235292,
                min_y: 191.73914,
            }),
        ),
        (
            ItemId(1049),
            Item::Rect(Rect {
                max_x: 14.558823,
                max_y: 191.73914,
                min_x: 11.6470585,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(1050),
            Item::Rect(Rect {
                max_x: 16.014706,
                max_y: 182.6087,
                min_x: 14.558823,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1051),
            Item::Rect(Rect {
                max_x: 16.014706,
                max_y: 173.47827,
                min_x: 11.6470585,
                min_y: 166.63043,
            }),
        ),
        (
            ItemId(1052),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 166.63043,
                min_x: 8.735294,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(1053),
            Item::Rect(Rect {
                max_x: 8.735294,
                max_y: 164.34782,
                min_x: 5.8235292,
                min_y: 162.06522,
            }),
        ),
        (
            ItemId(1054),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 162.06522,
                min_x: 0.0,
                min_y: 159.78261,
            }),
        ),
        (
            ItemId(1055),
            Item::Rect(Rect {
                max_x: 0.0,
                max_y: 182.6087,
                min_x: 0.0,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(1056),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 6.847826,
                min_x: 279.52942,
                min_y: 3.0434785,
            }),
        ),
        (
            ItemId(1057),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 8.454106,
                min_x: 221.72548,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1058),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 3.0434785,
                min_x: 277.58823,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1059),
            Item::Rect(Rect {
                max_x: 34.941177,
                max_y: 91.30435,
                min_x: 29.117645,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(1060),
            Item::Rect(Rect {
                max_x: 29.117645,
                max_y: 91.30435,
                min_x: 23.294117,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(1061),
            Item::Rect(Rect {
                max_x: 29.117645,
                max_y: 95.86957,
                min_x: 23.294117,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(1062),
            Item::Rect(Rect {
                max_x: 34.941177,
                max_y: 95.86957,
                min_x: 29.117645,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(1063),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 109.565216,
                min_x: 250.41176,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1064),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 91.30435,
                min_x: 238.7647,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(1065),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 91.30435,
                min_x: 244.58823,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(1066),
            Item::Rect(Rect {
                max_x: 253.32352,
                max_y: 91.30435,
                min_x: 250.41176,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(1067),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 82.17391,
                min_x: 253.32352,
                min_y: 79.13044,
            }),
        ),
        (
            ItemId(1068),
            Item::Rect(Rect {
                max_x: 260.11765,
                max_y: 79.13044,
                min_x: 256.2353,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(1069),
            Item::Rect(Rect {
                max_x: 260.11765,
                max_y: 73.04348,
                min_x: 256.2353,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(1070),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 66.95653,
                min_x: 250.41176,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(1071),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 73.04348,
                min_x: 244.58823,
                min_y: 66.95653,
            }),
        ),
        (
            ItemId(1072),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 82.17391,
                min_x: 238.7647,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(1073),
            Item::Rect(Rect {
                max_x: 93.17647,
                max_y: 146.08696,
                min_x: 93.17647,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1074),
            Item::Rect(Rect {
                max_x: 93.17647,
                max_y: 146.08696,
                min_x: 87.352936,
                min_y: 141.52174,
            }),
        ),
        (
            ItemId(1075),
            Item::Rect(Rect {
                max_x: 87.352936,
                max_y: 143.04349,
                min_x: 81.52941,
                min_y: 141.52174,
            }),
        ),
        (
            ItemId(1076),
            Item::Rect(Rect {
                max_x: 81.52941,
                max_y: 146.08696,
                min_x: 78.617645,
                min_y: 143.04349,
            }),
        ),
        (
            ItemId(1077),
            Item::Rect(Rect {
                max_x: 81.52941,
                max_y: 155.21739,
                min_x: 78.617645,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1078),
            Item::Rect(Rect {
                max_x: 87.352936,
                max_y: 155.21739,
                min_x: 81.52941,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(1079),
            Item::Rect(Rect {
                max_x: 87.352936,
                max_y: 155.21739,
                min_x: 87.352936,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(1080),
            Item::Rect(Rect {
                max_x: 93.17647,
                max_y: 159.78261,
                min_x: 87.352936,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(1081),
            Item::Rect(Rect {
                max_x: 99.0,
                max_y: 159.78261,
                min_x: 93.17647,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(1082),
            Item::Rect(Rect {
                max_x: 99.0,
                max_y: 155.21739,
                min_x: 93.17647,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1083),
            Item::Rect(Rect {
                max_x: 198.0,
                max_y: 164.34782,
                min_x: 192.17647,
                min_y: 159.78261,
            }),
        ),
        (
            ItemId(1084),
            Item::Rect(Rect {
                max_x: 192.17647,
                max_y: 168.91304,
                min_x: 186.35294,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(1085),
            Item::Rect(Rect {
                max_x: 186.35294,
                max_y: 173.47827,
                min_x: 183.44118,
                min_y: 168.91304,
            }),
        ),
        (
            ItemId(1086),
            Item::Rect(Rect {
                max_x: 186.35294,
                max_y: 176.52174,
                min_x: 183.44118,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1087),
            Item::Rect(Rect {
                max_x: 192.17647,
                max_y: 179.56522,
                min_x: 186.35294,
                min_y: 176.52174,
            }),
        ),
        (
            ItemId(1088),
            Item::Rect(Rect {
                max_x: 198.0,
                max_y: 179.56522,
                min_x: 192.17647,
                min_y: 176.52174,
            }),
        ),
        (
            ItemId(1089),
            Item::Rect(Rect {
                max_x: 200.91176,
                max_y: 176.52174,
                min_x: 198.0,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1090),
            Item::Rect(Rect {
                max_x: 203.82352,
                max_y: 173.47827,
                min_x: 200.91176,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(1091),
            Item::Rect(Rect {
                max_x: 203.82352,
                max_y: 164.34782,
                min_x: 198.0,
                min_y: 159.78261,
            }),
        ),
        (
            ItemId(1092),
            Item::Rect(Rect {
                max_x: 64.05882,
                max_y: 114.13044,
                min_x: 58.23529,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1093),
            Item::Rect(Rect {
                max_x: 69.882355,
                max_y: 114.13044,
                min_x: 64.05882,
                min_y: 114.13044,
            }),
        ),
        (
            ItemId(1094),
            Item::Rect(Rect {
                max_x: 75.70588,
                max_y: 114.13044,
                min_x: 69.882355,
                min_y: 114.13044,
            }),
        ),
        (
            ItemId(1095),
            Item::Rect(Rect {
                max_x: 81.52941,
                max_y: 118.695656,
                min_x: 75.70588,
                min_y: 114.13044,
            }),
        ),
        (
            ItemId(1096),
            Item::Rect(Rect {
                max_x: 87.352936,
                max_y: 118.695656,
                min_x: 81.52941,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1097),
            Item::Rect(Rect {
                max_x: 87.352936,
                max_y: 109.565216,
                min_x: 81.52941,
                min_y: 103.478264,
            }),
        ),
        (
            ItemId(1098),
            Item::Rect(Rect {
                max_x: 81.52941,
                max_y: 106.52174,
                min_x: 75.70588,
                min_y: 103.478264,
            }),
        ),
        (
            ItemId(1099),
            Item::Rect(Rect {
                max_x: 75.70588,
                max_y: 106.52174,
                min_x: 69.882355,
                min_y: 106.52174,
            }),
        ),
        (
            ItemId(1100),
            Item::Rect(Rect {
                max_x: 69.882355,
                max_y: 106.52174,
                min_x: 64.05882,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(1101),
            Item::Rect(Rect {
                max_x: 64.05882,
                max_y: 109.565216,
                min_x: 58.23529,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(1102),
            Item::Rect(Rect {
                max_x: 284.18823,
                max_y: 37.874397,
                min_x: 279.52942,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(1103),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 37.874397,
                min_x: 278.66666,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(1104),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 36.52174,
                min_x: 278.66666,
                min_y: 29.217392,
            }),
        ),
        (
            ItemId(1105),
            Item::Rect(Rect {
                max_x: 284.18823,
                max_y: 36.52174,
                min_x: 279.52942,
                min_y: 29.217392,
            }),
        ),
        (
            ItemId(1106),
            Item::Rect(Rect {
                max_x: 157.23529,
                max_y: 54.782608,
                min_x: 151.41176,
                min_y: 51.739132,
            }),
        ),
        (
            ItemId(1107),
            Item::Rect(Rect {
                max_x: 157.23529,
                max_y: 63.913044,
                min_x: 151.41176,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(1108),
            Item::Rect(Rect {
                max_x: 157.23529,
                max_y: 63.913044,
                min_x: 157.23529,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(1109),
            Item::Rect(Rect {
                max_x: 163.05882,
                max_y: 73.04348,
                min_x: 157.23529,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(1110),
            Item::Rect(Rect {
                max_x: 168.88235,
                max_y: 73.04348,
                min_x: 163.05882,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(1111),
            Item::Rect(Rect {
                max_x: 174.70587,
                max_y: 73.04348,
                min_x: 168.88235,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(1112),
            Item::Rect(Rect {
                max_x: 174.70587,
                max_y: 63.913044,
                min_x: 168.88235,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(1113),
            Item::Rect(Rect {
                max_x: 168.88235,
                max_y: 54.782608,
                min_x: 163.05882,
                min_y: 51.739132,
            }),
        ),
        (
            ItemId(1114),
            Item::Rect(Rect {
                max_x: 163.05882,
                max_y: 51.739132,
                min_x: 157.23529,
                min_y: 51.739132,
            }),
        ),
        (
            ItemId(1115),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 132.39131,
                min_x: 291.01907,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1116),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 127.82609,
                min_x: 291.01907,
                min_y: 127.46088,
            }),
        ),
        (
            ItemId(1117),
            Item::Rect(Rect {
                max_x: 192.17647,
                max_y: 82.17391,
                min_x: 189.2647,
                min_y: 77.608696,
            }),
        ),
        (
            ItemId(1118),
            Item::Rect(Rect {
                max_x: 192.17647,
                max_y: 86.739136,
                min_x: 189.2647,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(1119),
            Item::Rect(Rect {
                max_x: 198.0,
                max_y: 86.739136,
                min_x: 192.17647,
                min_y: 86.739136,
            }),
        ),
        (
            ItemId(1120),
            Item::Rect(Rect {
                max_x: 200.91176,
                max_y: 86.739136,
                min_x: 198.0,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(1121),
            Item::Rect(Rect {
                max_x: 200.91176,
                max_y: 82.17391,
                min_x: 198.0,
                min_y: 77.608696,
            }),
        ),
        (
            ItemId(1122),
            Item::Rect(Rect {
                max_x: 198.0,
                max_y: 77.608696,
                min_x: 192.17647,
                min_y: 77.608696,
            }),
        ),
        (
            ItemId(1123),
            Item::Rect(Rect {
                max_x: 286.2847,
                max_y: 118.695656,
                min_x: 285.58588,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1124),
            Item::Rect(Rect {
                max_x: 285.58588,
                max_y: 118.94242,
                min_x: 285.35294,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(1125),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 118.94242,
                min_x: 279.52942,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(1126),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 118.695656,
                min_x: 279.52942,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(1127),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 118.695656,
                min_x: 279.52942,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(1128),
            Item::Rect(Rect {
                max_x: 280.69412,
                max_y: 118.695656,
                min_x: 279.52942,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1129),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 109.565216,
                min_x: 280.69412,
                min_y: 108.21256,
            }),
        ),
        (
            ItemId(1130),
            Item::Rect(Rect {
                max_x: 286.2847,
                max_y: 109.565216,
                min_x: 285.35294,
                min_y: 108.21256,
            }),
        ),
        (
            ItemId(1131),
            Item::Rect(Rect {
                max_x: 170.82353,
                max_y: 164.34782,
                min_x: 168.88235,
                min_y: 159.78261,
            }),
        ),
        (
            ItemId(1132),
            Item::Rect(Rect {
                max_x: 168.88235,
                max_y: 164.34782,
                min_x: 163.05882,
                min_y: 159.78261,
            }),
        ),
        (
            ItemId(1133),
            Item::Rect(Rect {
                max_x: 168.88235,
                max_y: 173.47827,
                min_x: 163.05882,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(1134),
            Item::Rect(Rect {
                max_x: 170.82353,
                max_y: 173.47827,
                min_x: 168.88235,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(1135),
            Item::Rect(Rect {
                max_x: 273.0588,
                max_y: 18.26087,
                min_x: 267.88235,
                min_y: 10.956522,
            }),
        ),
        (
            ItemId(1136),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 10.956522,
                min_x: 266.7552,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1137),
            Item::Rect(Rect {
                max_x: 266.7552,
                max_y: 9.130435,
                min_x: 262.0588,
                min_y: 1.2593703,
            }),
        ),
        (
            ItemId(1138),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 54.782608,
                min_x: 290.9435,
                min_y: 52.5,
            }),
        ),
        (
            ItemId(1139),
            Item::Rect(Rect {
                max_x: 278.5588,
                max_y: 29.02174,
                min_x: 273.70587,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(1140),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 29.02174,
                min_x: 272.58597,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(1141),
            Item::Rect(Rect {
                max_x: 273.0588,
                max_y: 27.391304,
                min_x: 272.58597,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1142),
            Item::Rect(Rect {
                max_x: 168.88235,
                max_y: 54.782608,
                min_x: 168.88235,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(1143),
            Item::Rect(Rect {
                max_x: 115.5,
                max_y: 116.928474,
                min_x: 110.64706,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1144),
            Item::Rect(Rect {
                max_x: 115.5,
                max_y: 109.565216,
                min_x: 110.64706,
                min_y: 101.69415,
            }),
        ),
        (
            ItemId(1145),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 109.565216,
                min_x: 105.95066,
                min_y: 101.69415,
            }),
        ),
        (
            ItemId(1146),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 116.928474,
                min_x: 105.95066,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1147),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 27.391304,
                min_x: 244.58823,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(1148),
            Item::Rect(Rect {
                max_x: 288.2647,
                max_y: 18.26087,
                min_x: 285.35294,
                min_y: 11.413044,
            }),
        ),
        (
            ItemId(1149),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 12.782609,
                min_x: 279.52942,
                min_y: 11.413044,
            }),
        ),
        (
            ItemId(1150),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 18.26087,
                min_x: 276.61765,
                min_y: 12.782609,
            }),
        ),
        (
            ItemId(1151),
            Item::Rect(Rect {
                max_x: 278.5588,
                max_y: 27.391304,
                min_x: 276.61765,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1152),
            Item::Rect(Rect {
                max_x: 288.2647,
                max_y: 27.391304,
                min_x: 287.2941,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1153),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 55.13378,
                min_x: 290.9435,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(1154),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 100.434784,
                min_x: 279.52942,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(1155),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 73.04348,
                min_x: 5.8235292,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(1156),
            Item::Rect(Rect {
                max_x: 287.68234,
                max_y: 45.652176,
                min_x: 285.35294,
                min_y: 39.56522,
            }),
        ),
        (
            ItemId(1157),
            Item::Rect(Rect {
                max_x: 287.68234,
                max_y: 46.354515,
                min_x: 285.35294,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(1158),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 46.354515,
                min_x: 284.88705,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(1159),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 45.652176,
                min_x: 284.88705,
                min_y: 39.56522,
            }),
        ),
        (
            ItemId(1160),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 136.95653,
                min_x: 262.0588,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1161),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 36.52174,
                min_x: 291.17645,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(1162),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 36.52174,
                min_x: 287.2941,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(1163),
            Item::Rect(Rect {
                max_x: 192.17647,
                max_y: 164.34782,
                min_x: 192.17647,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(1164),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 73.04348,
                min_x: 244.58823,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(1165),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 118.695656,
                min_x: 244.58823,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(1166),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 6.3210707,
                min_x: 285.35294,
                min_y: 2.2826087,
            }),
        ),
        (
            ItemId(1167),
            Item::Rect(Rect {
                max_x: 234.23528,
                max_y: 9.130435,
                min_x: 232.94116,
                min_y: 6.9391303,
            }),
        ),
        (
            ItemId(1168),
            Item::Rect(Rect {
                max_x: 238.7647,
                max_y: 16.505016,
                min_x: 234.23528,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1169),
            Item::Rect(Rect {
                max_x: 239.80461,
                max_y: 18.26087,
                min_x: 238.7647,
                min_y: 16.505016,
            }),
        ),
        (
            ItemId(1170),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 26.660872,
                min_x: 239.80461,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1171),
            Item::Rect(Rect {
                max_x: 245.09462,
                max_y: 27.391304,
                min_x: 244.58823,
                min_y: 26.660872,
            }),
        ),
        (
            ItemId(1172),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 34.003,
                min_x: 245.09462,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(1173),
            Item::Rect(Rect {
                max_x: 252.13724,
                max_y: 36.52174,
                min_x: 250.41176,
                min_y: 34.003,
            }),
        ),
        (
            ItemId(1174),
            Item::Rect(Rect {
                max_x: 252.13724,
                max_y: 45.652176,
                min_x: 251.81743,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(1175),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 53.091785,
                min_x: 251.81743,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(1176),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 53.091785,
                min_x: 256.2353,
                min_y: 52.077297,
            }),
        ),
        (
            ItemId(1177),
            Item::Rect(Rect {
                max_x: 263.7843,
                max_y: 54.782608,
                min_x: 262.0588,
                min_y: 52.077297,
            }),
        ),
        (
            ItemId(1178),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 61.45485,
                min_x: 263.7843,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(1179),
            Item::Rect(Rect {
                max_x: 269.45023,
                max_y: 63.913044,
                min_x: 267.88235,
                min_y: 61.45485,
            }),
        ),
        (
            ItemId(1180),
            Item::Rect(Rect {
                max_x: 269.45023,
                max_y: 73.04348,
                min_x: 268.7783,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(1181),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 81.41305,
                min_x: 268.7783,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(1182),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 81.41305,
                min_x: 273.70587,
                min_y: 79.468605,
            }),
        ),
        (
            ItemId(1183),
            Item::Rect(Rect {
                max_x: 281.25488,
                max_y: 82.17391,
                min_x: 279.52942,
                min_y: 79.468605,
            }),
        ),
        (
            ItemId(1184),
            Item::Rect(Rect {
                max_x: 281.25488,
                max_y: 91.30435,
                min_x: 280.69412,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(1185),
            Item::Rect(Rect {
                max_x: 280.69412,
                max_y: 100.434784,
                min_x: 280.0358,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(1186),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 107.53623,
                min_x: 280.0358,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(1187),
            Item::Rect(Rect {
                max_x: 286.75058,
                max_y: 109.565216,
                min_x: 285.35294,
                min_y: 107.53623,
            }),
        ),
        (
            ItemId(1188),
            Item::Rect(Rect {
                max_x: 286.75058,
                max_y: 118.695656,
                min_x: 286.05176,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1189),
            Item::Rect(Rect {
                max_x: 286.05176,
                max_y: 119.43597,
                min_x: 285.35294,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(1190),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 119.43597,
                min_x: 279.52942,
                min_y: 119.152176,
            }),
        ),
        (
            ItemId(1191),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 119.152176,
                min_x: 273.70587,
                min_y: 118.94242,
            }),
        ),
        (
            ItemId(1192),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 127.82609,
                min_x: 267.88235,
                min_y: 118.94242,
            }),
        ),
        (
            ItemId(1193),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 136.95653,
                min_x: 262.38235,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1194),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 145.8261,
                min_x: 262.38235,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1195),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 146.08696,
                min_x: 267.88235,
                min_y: 145.8261,
            }),
        ),
        (
            ItemId(1196),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 146.08696,
                min_x: 273.70587,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1197),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 155.21739,
                min_x: 273.70587,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1198),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 155.21739,
                min_x: 279.52942,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(1199),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 155.21739,
                min_x: 285.35294,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(1200),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 155.21739,
                min_x: 285.35294,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(1201),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 164.34782,
                min_x: 282.44116,
                min_y: 155.21739,
            }),
        ),
        (
            ItemId(1202),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 173.47827,
                min_x: 282.44116,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(1203),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 173.47827,
                min_x: 285.35294,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1204),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 178.04349,
                min_x: 285.35294,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1205),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 185.65219,
                min_x: 3.8823528,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(1206),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 185.65219,
                min_x: 5.8235292,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(1207),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 182.6087,
                min_x: 11.6470585,
                min_y: 182.6087,
            }),
        ),
        (
            ItemId(1208),
            Item::Rect(Rect {
                max_x: 13.102941,
                max_y: 182.6087,
                min_x: 11.6470585,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1209),
            Item::Rect(Rect {
                max_x: 13.102941,
                max_y: 173.47827,
                min_x: 11.6470585,
                min_y: 171.19566,
            }),
        ),
        (
            ItemId(1210),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 171.19566,
                min_x: 5.8235292,
                min_y: 164.60146,
            }),
        ),
        (
            ItemId(1211),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 173.47827,
                min_x: 0.1617647,
                min_y: 164.60146,
            }),
        ),
        (
            ItemId(1212),
            Item::Rect(Rect {
                max_x: 3.8823528,
                max_y: 182.6087,
                min_x: 0.1617647,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1213),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 2.3671498,
                min_x: 254.66742,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1214),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 7.777778,
                min_x: 227.11765,
                min_y: 6.9391303,
            }),
        ),
        (
            ItemId(1215),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 7.777778,
                min_x: 222.15686,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1216),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 6.774194,
                min_x: 0.0,
                min_y: 6.5217395,
            }),
        ),
        (
            ItemId(1217),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 6.6956525,
                min_x: 5.8235292,
                min_y: 6.5217395,
            }),
        ),
        (
            ItemId(1218),
            Item::Rect(Rect {
                max_x: 15.917646,
                max_y: 6.6956525,
                min_x: 11.6470585,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1219),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 2.3671498,
                min_x: 256.2353,
                min_y: 1.8890556,
            }),
        ),
        (
            ItemId(1220),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 86.739136,
                min_x: 2.9117646,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(1221),
            Item::Rect(Rect {
                max_x: 8.735294,
                max_y: 86.739136,
                min_x: 5.8235292,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(1222),
            Item::Rect(Rect {
                max_x: 8.735294,
                max_y: 82.17391,
                min_x: 5.8235292,
                min_y: 79.13044,
            }),
        ),
        (
            ItemId(1223),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 82.17391,
                min_x: 2.9117646,
                min_y: 79.13044,
            }),
        ),
        (
            ItemId(1224),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 2.2826087,
                min_x: 282.44116,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1225),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 127.82609,
                min_x: 290.70428,
                min_y: 126.73044,
            }),
        ),
        (
            ItemId(1226),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 22.826088,
                min_x: 278.5588,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1227),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 22.826088,
                min_x: 279.52942,
                min_y: 22.826088,
            }),
        ),
        (
            ItemId(1228),
            Item::Rect(Rect {
                max_x: 286.32352,
                max_y: 22.826088,
                min_x: 285.35294,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1229),
            Item::Rect(Rect {
                max_x: 286.32352,
                max_y: 18.26087,
                min_x: 285.35294,
                min_y: 15.978261,
            }),
        ),
        (
            ItemId(1230),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 16.434782,
                min_x: 279.52942,
                min_y: 15.978261,
            }),
        ),
        (
            ItemId(1231),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 18.26087,
                min_x: 278.5588,
                min_y: 16.434782,
            }),
        ),
        (
            ItemId(1232),
            Item::Rect(Rect {
                max_x: 266.3795,
                max_y: 9.130435,
                min_x: 262.0588,
                min_y: 1.8890556,
            }),
        ),
        (
            ItemId(1233),
            Item::Rect(Rect {
                max_x: 272.62744,
                max_y: 18.26087,
                min_x: 267.88235,
                min_y: 11.565218,
            }),
        ),
        (
            ItemId(1234),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 11.565218,
                min_x: 266.3795,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1235),
            Item::Rect(Rect {
                max_x: 115.11176,
                max_y: 116.33942,
                min_x: 110.64706,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1236),
            Item::Rect(Rect {
                max_x: 115.11176,
                max_y: 109.565216,
                min_x: 110.64706,
                min_y: 102.323845,
            }),
        ),
        (
            ItemId(1237),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 109.565216,
                min_x: 106.32637,
                min_y: 102.323845,
            }),
        ),
        (
            ItemId(1238),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 116.33942,
                min_x: 106.32637,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1239),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 127.82609,
                min_x: 267.88235,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1240),
            Item::Rect(Rect {
                max_x: 284.42117,
                max_y: 45.652176,
                min_x: 279.52942,
                min_y: 38.550728,
            }),
        ),
        (
            ItemId(1241),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 38.550728,
                min_x: 278.2353,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(1242),
            Item::Rect(Rect {
                max_x: 278.2353,
                max_y: 36.52174,
                min_x: 273.70587,
                min_y: 29.673914,
            }),
        ),
        (
            ItemId(1243),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 29.673914,
                min_x: 272.138,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(1244),
            Item::Rect(Rect {
                max_x: 272.62744,
                max_y: 27.391304,
                min_x: 272.138,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1245),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 146.08696,
                min_x: 291.17645,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1246),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 146.08696,
                min_x: 285.35294,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1247),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 136.95653,
                min_x: 285.35294,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1248),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 136.95653,
                min_x: 285.35294,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1249),
            Item::Rect(Rect {
                max_x: 290.70428,
                max_y: 136.95653,
                min_x: 285.35294,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1250),
            Item::Rect(Rect {
                max_x: 290.01175,
                max_y: 47.056858,
                min_x: 285.35294,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(1251),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 47.056858,
                min_x: 284.42117,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(1252),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 54.782608,
                min_x: 290.47763,
                min_y: 47.934784,
            }),
        ),
        (
            ItemId(1253),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 45.652176,
                min_x: 290.01175,
                min_y: 42.608696,
            }),
        ),
        (
            ItemId(1254),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 55.83612,
                min_x: 290.47763,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(1255),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 182.34016,
                min_x: 0.4852941,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1256),
            Item::Rect(Rect {
                max_x: 11.475778,
                max_y: 182.34016,
                min_x: 5.8235292,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1257),
            Item::Rect(Rect {
                max_x: 11.475778,
                max_y: 173.47827,
                min_x: 5.8235292,
                min_y: 165.1087,
            }),
        ),
        (
            ItemId(1258),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 173.47827,
                min_x: 0.4852941,
                min_y: 165.1087,
            }),
        ),
        (
            ItemId(1259),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 6.086957,
                min_x: 5.8235292,
                min_y: 5.8695655,
            }),
        ),
        (
            ItemId(1260),
            Item::Rect(Rect {
                max_x: 15.529411,
                max_y: 6.086957,
                min_x: 11.6470585,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1261),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 7.1014495,
                min_x: 222.58823,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1262),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 7.1014495,
                min_x: 227.11765,
                min_y: 6.208696,
            }),
        ),
        (
            ItemId(1263),
            Item::Rect(Rect {
                max_x: 234.66666,
                max_y: 9.130435,
                min_x: 232.94116,
                min_y: 6.208696,
            }),
        ),
        (
            ItemId(1264),
            Item::Rect(Rect {
                max_x: 238.7647,
                max_y: 15.802675,
                min_x: 234.66666,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1265),
            Item::Rect(Rect {
                max_x: 240.22058,
                max_y: 18.26087,
                min_x: 238.7647,
                min_y: 15.802675,
            }),
        ),
        (
            ItemId(1266),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 25.930435,
                min_x: 240.22058,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1267),
            Item::Rect(Rect {
                max_x: 245.60101,
                max_y: 27.391304,
                min_x: 244.58823,
                min_y: 25.930435,
            }),
        ),
        (
            ItemId(1268),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 33.373314,
                min_x: 245.60101,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(1269),
            Item::Rect(Rect {
                max_x: 252.56862,
                max_y: 36.52174,
                min_x: 250.41176,
                min_y: 33.373314,
            }),
        ),
        (
            ItemId(1270),
            Item::Rect(Rect {
                max_x: 252.56862,
                max_y: 45.652176,
                min_x: 252.21906,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(1271),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 52.41546,
                min_x: 252.21906,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(1272),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 52.41546,
                min_x: 256.2353,
                min_y: 51.400967,
            }),
        ),
        (
            ItemId(1273),
            Item::Rect(Rect {
                max_x: 264.21567,
                max_y: 54.782608,
                min_x: 262.0588,
                min_y: 51.400967,
            }),
        ),
        (
            ItemId(1274),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 60.75251,
                min_x: 264.21567,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(1275),
            Item::Rect(Rect {
                max_x: 269.8982,
                max_y: 63.913044,
                min_x: 267.88235,
                min_y: 60.75251,
            }),
        ),
        (
            ItemId(1276),
            Item::Rect(Rect {
                max_x: 269.8982,
                max_y: 73.04348,
                min_x: 269.22623,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(1277),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 80.652176,
                min_x: 269.22623,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(1278),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 80.652176,
                min_x: 273.70587,
                min_y: 78.79227,
            }),
        ),
        (
            ItemId(1279),
            Item::Rect(Rect {
                max_x: 281.68625,
                max_y: 82.17391,
                min_x: 279.52942,
                min_y: 78.79227,
            }),
        ),
        (
            ItemId(1280),
            Item::Rect(Rect {
                max_x: 281.68625,
                max_y: 91.30435,
                min_x: 281.15997,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(1281),
            Item::Rect(Rect {
                max_x: 281.15997,
                max_y: 100.434784,
                min_x: 280.54218,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(1282),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 106.85991,
                min_x: 280.54218,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(1283),
            Item::Rect(Rect {
                max_x: 287.21646,
                max_y: 109.565216,
                min_x: 285.35294,
                min_y: 106.85991,
            }),
        ),
        (
            ItemId(1284),
            Item::Rect(Rect {
                max_x: 287.21646,
                max_y: 118.695656,
                min_x: 286.51764,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1285),
            Item::Rect(Rect {
                max_x: 286.51764,
                max_y: 119.9295,
                min_x: 285.35294,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(1286),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 119.9295,
                min_x: 279.52942,
                min_y: 119.6087,
            }),
        ),
        (
            ItemId(1287),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 119.6087,
                min_x: 273.70587,
                min_y: 119.43597,
            }),
        ),
        (
            ItemId(1288),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 127.82609,
                min_x: 268.20587,
                min_y: 119.43597,
            }),
        ),
        (
            ItemId(1289),
            Item::Rect(Rect {
                max_x: 268.20587,
                max_y: 128.36317,
                min_x: 267.88235,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1290),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 136.95653,
                min_x: 262.70587,
                min_y: 128.36317,
            }),
        ),
        (
            ItemId(1291),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 145.30435,
                min_x: 262.70587,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1292),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 145.59341,
                min_x: 267.88235,
                min_y: 145.30435,
            }),
        ),
        (
            ItemId(1293),
            Item::Rect(Rect {
                max_x: 274.04843,
                max_y: 146.08696,
                min_x: 273.70587,
                min_y: 145.59341,
            }),
        ),
        (
            ItemId(1294),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 154.68031,
                min_x: 274.04843,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1295),
            Item::Rect(Rect {
                max_x: 285.17645,
                max_y: 154.68031,
                min_x: 279.52942,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1296),
            Item::Rect(Rect {
                max_x: 285.17645,
                max_y: 146.08696,
                min_x: 285.0294,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1297),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 136.95653,
                min_x: 285.0294,
                min_y: 136.41943,
            }),
        ),
        (
            ItemId(1298),
            Item::Rect(Rect {
                max_x: 290.3895,
                max_y: 136.41943,
                min_x: 285.35294,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1299),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 127.82609,
                min_x: 290.3895,
                min_y: 126.00001,
            }),
        ),
        (
            ItemId(1300),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 3.0434785,
                min_x: 256.2353,
                min_y: 2.5187407,
            }),
        ),
        (
            ItemId(1301),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 3.0434785,
                min_x: 254.21945,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1302),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 6.1851335,
                min_x: 0.0,
                min_y: 5.8695655,
            }),
        ),
        (
            ItemId(1303),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 4.9163885,
                min_x: 286.08087,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1304),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 12.173914,
                min_x: 266.00378,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1305),
            Item::Rect(Rect {
                max_x: 266.00378,
                max_y: 9.130435,
                min_x: 262.0588,
                min_y: 2.5187407,
            }),
        ),
        (
            ItemId(1306),
            Item::Rect(Rect {
                max_x: 114.72353,
                max_y: 115.75036,
                min_x: 110.64706,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1307),
            Item::Rect(Rect {
                max_x: 114.72353,
                max_y: 109.565216,
                min_x: 110.64706,
                min_y: 102.95352,
            }),
        ),
        (
            ItemId(1308),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 109.565216,
                min_x: 106.70208,
                min_y: 102.95352,
            }),
        ),
        (
            ItemId(1309),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 115.75036,
                min_x: 106.70208,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1310),
            Item::Rect(Rect {
                max_x: 290.01175,
                max_y: 54.782608,
                min_x: 285.35294,
                min_y: 47.759197,
            }),
        ),
        (
            ItemId(1311),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 47.759197,
                min_x: 283.95526,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(1312),
            Item::Rect(Rect {
                max_x: 283.95526,
                max_y: 45.652176,
                min_x: 279.52942,
                min_y: 39.22705,
            }),
        ),
        (
            ItemId(1313),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 39.22705,
                min_x: 277.80392,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(1314),
            Item::Rect(Rect {
                max_x: 277.80392,
                max_y: 36.52174,
                min_x: 273.70587,
                min_y: 30.326088,
            }),
        ),
        (
            ItemId(1315),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 30.326088,
                min_x: 271.69003,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(1316),
            Item::Rect(Rect {
                max_x: 272.19608,
                max_y: 27.391304,
                min_x: 271.69003,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1317),
            Item::Rect(Rect {
                max_x: 272.19608,
                max_y: 18.26087,
                min_x: 267.88235,
                min_y: 12.173914,
            }),
        ),
        (
            ItemId(1318),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 56.53846,
                min_x: 290.01175,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(1319),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 173.20158,
                min_x: 285.52942,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(1320),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 164.34782,
                min_x: 285.52942,
                min_y: 155.49408,
            }),
        ),
        (
            ItemId(1321),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 6.425121,
                min_x: 227.11765,
                min_y: 5.478261,
            }),
        ),
        (
            ItemId(1322),
            Item::Rect(Rect {
                max_x: 235.09802,
                max_y: 9.130435,
                min_x: 232.94116,
                min_y: 5.478261,
            }),
        ),
        (
            ItemId(1323),
            Item::Rect(Rect {
                max_x: 238.7647,
                max_y: 15.100335,
                min_x: 235.09802,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1324),
            Item::Rect(Rect {
                max_x: 240.63655,
                max_y: 18.26087,
                min_x: 238.7647,
                min_y: 15.100335,
            }),
        ),
        (
            ItemId(1325),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 25.2,
                min_x: 240.63655,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1326),
            Item::Rect(Rect {
                max_x: 246.1074,
                max_y: 27.391304,
                min_x: 244.58823,
                min_y: 25.2,
            }),
        ),
        (
            ItemId(1327),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 32.74363,
                min_x: 246.1074,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(1328),
            Item::Rect(Rect {
                max_x: 252.99998,
                max_y: 36.52174,
                min_x: 250.41176,
                min_y: 32.74363,
            }),
        ),
        (
            ItemId(1329),
            Item::Rect(Rect {
                max_x: 252.99998,
                max_y: 45.652176,
                min_x: 252.62068,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(1330),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 51.739132,
                min_x: 252.62068,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(1331),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 51.739132,
                min_x: 256.2353,
                min_y: 50.724636,
            }),
        ),
        (
            ItemId(1332),
            Item::Rect(Rect {
                max_x: 264.64703,
                max_y: 54.782608,
                min_x: 262.0588,
                min_y: 50.724636,
            }),
        ),
        (
            ItemId(1333),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 60.050167,
                min_x: 264.64703,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(1334),
            Item::Rect(Rect {
                max_x: 270.34613,
                max_y: 63.913044,
                min_x: 267.88235,
                min_y: 60.050167,
            }),
        ),
        (
            ItemId(1335),
            Item::Rect(Rect {
                max_x: 270.34613,
                max_y: 73.04348,
                min_x: 269.6742,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(1336),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 79.891304,
                min_x: 269.6742,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(1337),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 79.891304,
                min_x: 273.70587,
                min_y: 78.115944,
            }),
        ),
        (
            ItemId(1338),
            Item::Rect(Rect {
                max_x: 282.1176,
                max_y: 82.17391,
                min_x: 279.52942,
                min_y: 78.115944,
            }),
        ),
        (
            ItemId(1339),
            Item::Rect(Rect {
                max_x: 282.1176,
                max_y: 91.30435,
                min_x: 281.6259,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(1340),
            Item::Rect(Rect {
                max_x: 281.6259,
                max_y: 100.434784,
                min_x: 281.04858,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(1341),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 106.18357,
                min_x: 281.04858,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(1342),
            Item::Rect(Rect {
                max_x: 287.68234,
                max_y: 109.565216,
                min_x: 285.35294,
                min_y: 106.18357,
            }),
        ),
        (
            ItemId(1343),
            Item::Rect(Rect {
                max_x: 287.68234,
                max_y: 118.695656,
                min_x: 286.98352,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1344),
            Item::Rect(Rect {
                max_x: 286.98352,
                max_y: 120.423035,
                min_x: 285.35294,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(1345),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 120.423035,
                min_x: 279.52942,
                min_y: 120.065216,
            }),
        ),
        (
            ItemId(1346),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 120.065216,
                min_x: 273.70587,
                min_y: 119.9295,
            }),
        ),
        (
            ItemId(1347),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 127.82609,
                min_x: 268.5294,
                min_y: 119.9295,
            }),
        ),
        (
            ItemId(1348),
            Item::Rect(Rect {
                max_x: 268.5294,
                max_y: 128.90025,
                min_x: 267.88235,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1349),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 136.95653,
                min_x: 263.02942,
                min_y: 128.90025,
            }),
        ),
        (
            ItemId(1350),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 144.78261,
                min_x: 263.02942,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1351),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 145.09988,
                min_x: 267.88235,
                min_y: 144.78261,
            }),
        ),
        (
            ItemId(1352),
            Item::Rect(Rect {
                max_x: 274.391,
                max_y: 146.08696,
                min_x: 273.70587,
                min_y: 145.09988,
            }),
        ),
        (
            ItemId(1353),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 154.14322,
                min_x: 274.391,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1354),
            Item::Rect(Rect {
                max_x: 284.82352,
                max_y: 154.14322,
                min_x: 279.52942,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1355),
            Item::Rect(Rect {
                max_x: 284.82352,
                max_y: 146.08696,
                min_x: 284.70587,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1356),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 136.95653,
                min_x: 284.70587,
                min_y: 135.88235,
            }),
        ),
        (
            ItemId(1357),
            Item::Rect(Rect {
                max_x: 290.0747,
                max_y: 135.88235,
                min_x: 285.35294,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1358),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 127.82609,
                min_x: 290.0747,
                min_y: 125.26957,
            }),
        ),
        (
            ItemId(1359),
            Item::Rect(Rect {
                max_x: 15.141175,
                max_y: 5.478261,
                min_x: 11.6470585,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1360),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 5.478261,
                min_x: 5.8235292,
                min_y: 5.2173915,
            }),
        ),
        (
            ItemId(1361),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 5.596073,
                min_x: 0.0,
                min_y: 5.2173915,
            }),
        ),
        (
            ItemId(1362),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 6.425121,
                min_x: 223.01959,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1363),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 3.719807,
                min_x: 253.77148,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1364),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 3.5117059,
                min_x: 287.53674,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1365),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 181.80307,
                min_x: 0.8088235,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1366),
            Item::Rect(Rect {
                max_x: 11.133217,
                max_y: 181.80307,
                min_x: 5.8235292,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1367),
            Item::Rect(Rect {
                max_x: 11.133217,
                max_y: 173.47827,
                min_x: 5.8235292,
                min_y: 165.61595,
            }),
        ),
        (
            ItemId(1368),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 173.47827,
                min_x: 0.8088235,
                min_y: 165.61595,
            }),
        ),
        (
            ItemId(1369),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 3.719807,
                min_x: 256.2353,
                min_y: 3.1484258,
            }),
        ),
        (
            ItemId(1370),
            Item::Rect(Rect {
                max_x: 271.7647,
                max_y: 27.391304,
                min_x: 271.24207,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1371),
            Item::Rect(Rect {
                max_x: 271.7647,
                max_y: 18.26087,
                min_x: 267.88235,
                min_y: 12.782609,
            }),
        ),
        (
            ItemId(1372),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 12.782609,
                min_x: 265.62808,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1373),
            Item::Rect(Rect {
                max_x: 265.62808,
                max_y: 9.130435,
                min_x: 262.0588,
                min_y: 3.1484258,
            }),
        ),
        (
            ItemId(1374),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 115.16129,
                min_x: 107.0778,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1375),
            Item::Rect(Rect {
                max_x: 114.33529,
                max_y: 115.16129,
                min_x: 110.64706,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1376),
            Item::Rect(Rect {
                max_x: 114.33529,
                max_y: 109.565216,
                min_x: 110.64706,
                min_y: 103.58321,
            }),
        ),
        (
            ItemId(1377),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 109.565216,
                min_x: 107.0778,
                min_y: 103.58321,
            }),
        ),
        (
            ItemId(1378),
            Item::Rect(Rect {
                max_x: 289.54587,
                max_y: 54.782608,
                min_x: 285.35294,
                min_y: 48.46154,
            }),
        ),
        (
            ItemId(1379),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 48.46154,
                min_x: 283.4894,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(1380),
            Item::Rect(Rect {
                max_x: 283.4894,
                max_y: 45.652176,
                min_x: 279.52942,
                min_y: 39.90338,
            }),
        ),
        (
            ItemId(1381),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 39.90338,
                min_x: 277.37256,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(1382),
            Item::Rect(Rect {
                max_x: 277.37256,
                max_y: 36.52174,
                min_x: 273.70587,
                min_y: 30.97826,
            }),
        ),
        (
            ItemId(1383),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 30.97826,
                min_x: 271.24207,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(1384),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 172.64822,
                min_x: 285.88232,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(1385),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 164.34782,
                min_x: 285.88232,
                min_y: 156.04744,
            }),
        ),
        (
            ItemId(1386),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 57.240803,
                min_x: 289.54587,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(1387),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 5.748792,
                min_x: 227.11765,
                min_y: 4.747826,
            }),
        ),
        (
            ItemId(1388),
            Item::Rect(Rect {
                max_x: 235.52939,
                max_y: 9.130435,
                min_x: 232.94116,
                min_y: 4.747826,
            }),
        ),
        (
            ItemId(1389),
            Item::Rect(Rect {
                max_x: 238.7647,
                max_y: 14.397994,
                min_x: 235.52939,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1390),
            Item::Rect(Rect {
                max_x: 241.0525,
                max_y: 18.26087,
                min_x: 238.7647,
                min_y: 14.397994,
            }),
        ),
        (
            ItemId(1391),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 24.469566,
                min_x: 241.0525,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1392),
            Item::Rect(Rect {
                max_x: 246.61382,
                max_y: 27.391304,
                min_x: 244.58823,
                min_y: 24.469566,
            }),
        ),
        (
            ItemId(1393),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 32.113945,
                min_x: 246.61382,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(1394),
            Item::Rect(Rect {
                max_x: 253.43138,
                max_y: 36.52174,
                min_x: 250.41176,
                min_y: 32.113945,
            }),
        ),
        (
            ItemId(1395),
            Item::Rect(Rect {
                max_x: 253.43138,
                max_y: 45.652176,
                min_x: 253.02231,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(1396),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 51.062805,
                min_x: 253.02231,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(1397),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 51.062805,
                min_x: 256.2353,
                min_y: 50.04831,
            }),
        ),
        (
            ItemId(1398),
            Item::Rect(Rect {
                max_x: 265.07843,
                max_y: 54.782608,
                min_x: 262.0588,
                min_y: 50.04831,
            }),
        ),
        (
            ItemId(1399),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 59.347828,
                min_x: 265.07843,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(1400),
            Item::Rect(Rect {
                max_x: 270.7941,
                max_y: 63.913044,
                min_x: 267.88235,
                min_y: 59.347828,
            }),
        ),
        (
            ItemId(1401),
            Item::Rect(Rect {
                max_x: 270.7941,
                max_y: 73.04348,
                min_x: 270.12216,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(1402),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 79.13044,
                min_x: 270.12216,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(1403),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 79.13044,
                min_x: 273.70587,
                min_y: 77.43961,
            }),
        ),
        (
            ItemId(1404),
            Item::Rect(Rect {
                max_x: 282.549,
                max_y: 82.17391,
                min_x: 279.52942,
                min_y: 77.43961,
            }),
        ),
        (
            ItemId(1405),
            Item::Rect(Rect {
                max_x: 282.549,
                max_y: 91.30435,
                min_x: 282.09174,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(1406),
            Item::Rect(Rect {
                max_x: 282.09174,
                max_y: 100.434784,
                min_x: 281.555,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(1407),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 105.50725,
                min_x: 281.555,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(1408),
            Item::Rect(Rect {
                max_x: 288.14822,
                max_y: 109.565216,
                min_x: 285.35294,
                min_y: 105.50725,
            }),
        ),
        (
            ItemId(1409),
            Item::Rect(Rect {
                max_x: 288.14822,
                max_y: 118.695656,
                min_x: 287.4494,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1410),
            Item::Rect(Rect {
                max_x: 287.4494,
                max_y: 120.91657,
                min_x: 285.35294,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(1411),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 120.91657,
                min_x: 279.52942,
                min_y: 120.52174,
            }),
        ),
        (
            ItemId(1412),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 120.52174,
                min_x: 273.70587,
                min_y: 120.423035,
            }),
        ),
        (
            ItemId(1413),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 127.82609,
                min_x: 268.85294,
                min_y: 120.423035,
            }),
        ),
        (
            ItemId(1414),
            Item::Rect(Rect {
                max_x: 268.85294,
                max_y: 129.43735,
                min_x: 267.88235,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1415),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 136.95653,
                min_x: 263.35294,
                min_y: 129.43735,
            }),
        ),
        (
            ItemId(1416),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 144.26088,
                min_x: 263.35294,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1417),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 144.60635,
                min_x: 267.88235,
                min_y: 144.26088,
            }),
        ),
        (
            ItemId(1418),
            Item::Rect(Rect {
                max_x: 274.73355,
                max_y: 146.08696,
                min_x: 273.70587,
                min_y: 144.60635,
            }),
        ),
        (
            ItemId(1419),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 153.60616,
                min_x: 274.73355,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1420),
            Item::Rect(Rect {
                max_x: 284.47058,
                max_y: 153.60616,
                min_x: 279.52942,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1421),
            Item::Rect(Rect {
                max_x: 284.47058,
                max_y: 146.08696,
                min_x: 284.38232,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1422),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 136.95653,
                min_x: 284.38232,
                min_y: 135.34528,
            }),
        ),
        (
            ItemId(1423),
            Item::Rect(Rect {
                max_x: 289.75992,
                max_y: 135.34528,
                min_x: 285.35294,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1424),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 127.82609,
                min_x: 289.75992,
                min_y: 124.53914,
            }),
        ),
        (
            ItemId(1425),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 5.007013,
                min_x: 0.0,
                min_y: 4.5652175,
            }),
        ),
        (
            ItemId(1426),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 4.8695655,
                min_x: 5.8235292,
                min_y: 4.5652175,
            }),
        ),
        (
            ItemId(1427),
            Item::Rect(Rect {
                max_x: 14.75294,
                max_y: 4.8695655,
                min_x: 11.6470585,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1428),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 4.3961353,
                min_x: 253.32352,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1429),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 5.748792,
                min_x: 223.45096,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1430),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 2.1070235,
                min_x: 288.99265,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1431),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 4.3961353,
                min_x: 256.2353,
                min_y: 3.7781112,
            }),
        ),
        (
            ItemId(1432),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 181.26599,
                min_x: 1.132353,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1433),
            Item::Rect(Rect {
                max_x: 10.790657,
                max_y: 181.26599,
                min_x: 5.8235292,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1434),
            Item::Rect(Rect {
                max_x: 10.790657,
                max_y: 173.47827,
                min_x: 5.8235292,
                min_y: 166.1232,
            }),
        ),
        (
            ItemId(1435),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 173.47827,
                min_x: 1.132353,
                min_y: 166.1232,
            }),
        ),
        (
            ItemId(1436),
            Item::Rect(Rect {
                max_x: 276.9412,
                max_y: 36.52174,
                min_x: 273.70587,
                min_y: 31.630434,
            }),
        ),
        (
            ItemId(1437),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 31.630434,
                min_x: 270.7941,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(1438),
            Item::Rect(Rect {
                max_x: 271.33334,
                max_y: 27.391304,
                min_x: 270.7941,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1439),
            Item::Rect(Rect {
                max_x: 271.33334,
                max_y: 18.26087,
                min_x: 267.88235,
                min_y: 13.391305,
            }),
        ),
        (
            ItemId(1440),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 13.391305,
                min_x: 265.25235,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1441),
            Item::Rect(Rect {
                max_x: 265.25235,
                max_y: 9.130435,
                min_x: 262.0588,
                min_y: 3.7781112,
            }),
        ),
        (
            ItemId(1442),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 109.565216,
                min_x: 107.45351,
                min_y: 104.2129,
            }),
        ),
        (
            ItemId(1443),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 114.572235,
                min_x: 107.45351,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1444),
            Item::Rect(Rect {
                max_x: 113.94706,
                max_y: 114.572235,
                min_x: 110.64706,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1445),
            Item::Rect(Rect {
                max_x: 113.94706,
                max_y: 109.565216,
                min_x: 110.64706,
                min_y: 104.2129,
            }),
        ),
        (
            ItemId(1446),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 40.579712,
                min_x: 276.9412,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(1447),
            Item::Rect(Rect {
                max_x: 289.08,
                max_y: 54.782608,
                min_x: 285.35294,
                min_y: 49.16388,
            }),
        ),
        (
            ItemId(1448),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 49.16388,
                min_x: 283.0235,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(1449),
            Item::Rect(Rect {
                max_x: 283.0235,
                max_y: 45.652176,
                min_x: 279.52942,
                min_y: 40.579712,
            }),
        ),
        (
            ItemId(1450),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 172.09486,
                min_x: 286.2353,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(1451),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 57.943146,
                min_x: 289.08,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(1452),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 164.34782,
                min_x: 286.2353,
                min_y: 156.6008,
            }),
        ),
        (
            ItemId(1453),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 4.417952,
                min_x: 0.0,
                min_y: 3.9130435,
            }),
        ),
        (
            ItemId(1454),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 4.2608695,
                min_x: 5.8235292,
                min_y: 3.9130435,
            }),
        ),
        (
            ItemId(1455),
            Item::Rect(Rect {
                max_x: 14.364706,
                max_y: 4.2608695,
                min_x: 11.6470585,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1456),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 5.072464,
                min_x: 227.11765,
                min_y: 4.017391,
            }),
        ),
        (
            ItemId(1457),
            Item::Rect(Rect {
                max_x: 235.96078,
                max_y: 9.130435,
                min_x: 232.94116,
                min_y: 4.017391,
            }),
        ),
        (
            ItemId(1458),
            Item::Rect(Rect {
                max_x: 238.7647,
                max_y: 13.695652,
                min_x: 235.96078,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1459),
            Item::Rect(Rect {
                max_x: 241.46849,
                max_y: 18.26087,
                min_x: 238.7647,
                min_y: 13.695652,
            }),
        ),
        (
            ItemId(1460),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 23.73913,
                min_x: 241.46849,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1461),
            Item::Rect(Rect {
                max_x: 247.12021,
                max_y: 27.391304,
                min_x: 244.58823,
                min_y: 23.73913,
            }),
        ),
        (
            ItemId(1462),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 31.484259,
                min_x: 247.12021,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(1463),
            Item::Rect(Rect {
                max_x: 253.86275,
                max_y: 36.52174,
                min_x: 250.41176,
                min_y: 31.484259,
            }),
        ),
        (
            ItemId(1464),
            Item::Rect(Rect {
                max_x: 253.86275,
                max_y: 45.652176,
                min_x: 253.42393,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(1465),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 50.386475,
                min_x: 253.42393,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(1466),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 50.386475,
                min_x: 256.2353,
                min_y: 49.371983,
            }),
        ),
        (
            ItemId(1467),
            Item::Rect(Rect {
                max_x: 265.5098,
                max_y: 54.782608,
                min_x: 262.0588,
                min_y: 49.371983,
            }),
        ),
        (
            ItemId(1468),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 58.64549,
                min_x: 265.5098,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(1469),
            Item::Rect(Rect {
                max_x: 271.24207,
                max_y: 63.913044,
                min_x: 267.88235,
                min_y: 58.64549,
            }),
        ),
        (
            ItemId(1470),
            Item::Rect(Rect {
                max_x: 271.24207,
                max_y: 73.04348,
                min_x: 270.57013,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(1471),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 78.36957,
                min_x: 270.57013,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(1472),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 78.36957,
                min_x: 273.70587,
                min_y: 76.76329,
            }),
        ),
        (
            ItemId(1473),
            Item::Rect(Rect {
                max_x: 282.9804,
                max_y: 82.17391,
                min_x: 279.52942,
                min_y: 76.76329,
            }),
        ),
        (
            ItemId(1474),
            Item::Rect(Rect {
                max_x: 282.9804,
                max_y: 91.30435,
                min_x: 282.55765,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(1475),
            Item::Rect(Rect {
                max_x: 282.55765,
                max_y: 100.434784,
                min_x: 282.06137,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(1476),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 104.83092,
                min_x: 282.06137,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(1477),
            Item::Rect(Rect {
                max_x: 288.6141,
                max_y: 109.565216,
                min_x: 285.35294,
                min_y: 104.83092,
            }),
        ),
        (
            ItemId(1478),
            Item::Rect(Rect {
                max_x: 288.6141,
                max_y: 118.695656,
                min_x: 287.91528,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1479),
            Item::Rect(Rect {
                max_x: 287.91528,
                max_y: 121.41011,
                min_x: 285.35294,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(1480),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 121.41011,
                min_x: 279.52942,
                min_y: 120.978264,
            }),
        ),
        (
            ItemId(1481),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 120.978264,
                min_x: 273.70587,
                min_y: 120.91657,
            }),
        ),
        (
            ItemId(1482),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 127.82609,
                min_x: 269.17645,
                min_y: 120.91657,
            }),
        ),
        (
            ItemId(1483),
            Item::Rect(Rect {
                max_x: 269.17645,
                max_y: 129.97443,
                min_x: 267.88235,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1484),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 136.95653,
                min_x: 263.67648,
                min_y: 129.97443,
            }),
        ),
        (
            ItemId(1485),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 143.73914,
                min_x: 263.67648,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1486),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 144.11281,
                min_x: 267.88235,
                min_y: 143.73914,
            }),
        ),
        (
            ItemId(1487),
            Item::Rect(Rect {
                max_x: 275.0761,
                max_y: 146.08696,
                min_x: 273.70587,
                min_y: 144.11281,
            }),
        ),
        (
            ItemId(1488),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 153.06906,
                min_x: 275.0761,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1489),
            Item::Rect(Rect {
                max_x: 284.11765,
                max_y: 153.06906,
                min_x: 279.52942,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1490),
            Item::Rect(Rect {
                max_x: 284.11765,
                max_y: 146.08696,
                min_x: 284.0588,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1491),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 136.95653,
                min_x: 284.0588,
                min_y: 134.80818,
            }),
        ),
        (
            ItemId(1492),
            Item::Rect(Rect {
                max_x: 289.44513,
                max_y: 134.80818,
                min_x: 285.35294,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1493),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 127.82609,
                min_x: 289.44513,
                min_y: 123.8087,
            }),
        ),
        (
            ItemId(1494),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 0.7023412,
                min_x: 290.44852,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1495),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 5.072464,
                min_x: 256.2353,
                min_y: 4.407796,
            }),
        ),
        (
            ItemId(1496),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 5.072464,
                min_x: 252.87555,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1497),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 180.7289,
                min_x: 1.4558823,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1498),
            Item::Rect(Rect {
                max_x: 10.448097,
                max_y: 180.7289,
                min_x: 5.8235292,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1499),
            Item::Rect(Rect {
                max_x: 10.448097,
                max_y: 173.47827,
                min_x: 5.8235292,
                min_y: 166.63043,
            }),
        ),
        (
            ItemId(1500),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 173.47827,
                min_x: 1.4558823,
                min_y: 166.63043,
            }),
        ),
        (
            ItemId(1501),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 5.072464,
                min_x: 223.88234,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1502),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 109.565216,
                min_x: 107.829216,
                min_y: 104.84258,
            }),
        ),
        (
            ItemId(1503),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 113.98317,
                min_x: 107.829216,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1504),
            Item::Rect(Rect {
                max_x: 113.55882,
                max_y: 113.98317,
                min_x: 110.64706,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1505),
            Item::Rect(Rect {
                max_x: 113.55882,
                max_y: 109.565216,
                min_x: 110.64706,
                min_y: 104.84258,
            }),
        ),
        (
            ItemId(1506),
            Item::Rect(Rect {
                max_x: 264.87665,
                max_y: 9.130435,
                min_x: 262.0588,
                min_y: 4.407796,
            }),
        ),
        (
            ItemId(1507),
            Item::Rect(Rect {
                max_x: 270.90198,
                max_y: 27.391304,
                min_x: 270.34613,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1508),
            Item::Rect(Rect {
                max_x: 270.90198,
                max_y: 18.26087,
                min_x: 267.88235,
                min_y: 14.0,
            }),
        ),
        (
            ItemId(1509),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 14.0,
                min_x: 264.87665,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1510),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 171.5415,
                min_x: 286.58823,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(1511),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 32.282608,
                min_x: 270.34613,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(1512),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 49.866222,
                min_x: 282.55765,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(1513),
            Item::Rect(Rect {
                max_x: 282.55765,
                max_y: 45.652176,
                min_x: 279.52942,
                min_y: 41.25604,
            }),
        ),
        (
            ItemId(1514),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 41.25604,
                min_x: 276.5098,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(1515),
            Item::Rect(Rect {
                max_x: 276.5098,
                max_y: 36.52174,
                min_x: 273.70587,
                min_y: 32.282608,
            }),
        ),
        (
            ItemId(1516),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 164.34782,
                min_x: 286.58823,
                min_y: 157.15416,
            }),
        ),
        (
            ItemId(1517),
            Item::Rect(Rect {
                max_x: 288.6141,
                max_y: 54.782608,
                min_x: 285.35294,
                min_y: 49.866222,
            }),
        ),
        (
            ItemId(1518),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 58.64549,
                min_x: 288.6141,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(1519),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 4.3961353,
                min_x: 224.31374,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1520),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 4.3961353,
                min_x: 227.11765,
                min_y: 3.2869568,
            }),
        ),
        (
            ItemId(1521),
            Item::Rect(Rect {
                max_x: 236.39217,
                max_y: 9.130435,
                min_x: 232.94116,
                min_y: 3.2869568,
            }),
        ),
        (
            ItemId(1522),
            Item::Rect(Rect {
                max_x: 238.7647,
                max_y: 12.993311,
                min_x: 236.39217,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1523),
            Item::Rect(Rect {
                max_x: 241.88445,
                max_y: 18.26087,
                min_x: 238.7647,
                min_y: 12.993311,
            }),
        ),
        (
            ItemId(1524),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 23.008696,
                min_x: 241.88445,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1525),
            Item::Rect(Rect {
                max_x: 247.6266,
                max_y: 27.391304,
                min_x: 244.58823,
                min_y: 23.008696,
            }),
        ),
        (
            ItemId(1526),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 30.854574,
                min_x: 247.6266,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(1527),
            Item::Rect(Rect {
                max_x: 254.29411,
                max_y: 36.52174,
                min_x: 250.41176,
                min_y: 30.854574,
            }),
        ),
        (
            ItemId(1528),
            Item::Rect(Rect {
                max_x: 254.29411,
                max_y: 45.652176,
                min_x: 253.82556,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(1529),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 49.710148,
                min_x: 253.82556,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(1530),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 49.710148,
                min_x: 256.2353,
                min_y: 48.695656,
            }),
        ),
        (
            ItemId(1531),
            Item::Rect(Rect {
                max_x: 265.94116,
                max_y: 54.782608,
                min_x: 262.0588,
                min_y: 48.695656,
            }),
        ),
        (
            ItemId(1532),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 57.943146,
                min_x: 265.94116,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(1533),
            Item::Rect(Rect {
                max_x: 271.69003,
                max_y: 63.913044,
                min_x: 267.88235,
                min_y: 57.943146,
            }),
        ),
        (
            ItemId(1534),
            Item::Rect(Rect {
                max_x: 271.69003,
                max_y: 73.04348,
                min_x: 271.01807,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(1535),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 77.608696,
                min_x: 271.01807,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(1536),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 77.608696,
                min_x: 273.70587,
                min_y: 76.08695,
            }),
        ),
        (
            ItemId(1537),
            Item::Rect(Rect {
                max_x: 283.41177,
                max_y: 82.17391,
                min_x: 279.52942,
                min_y: 76.08695,
            }),
        ),
        (
            ItemId(1538),
            Item::Rect(Rect {
                max_x: 283.41177,
                max_y: 91.30435,
                min_x: 283.0235,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(1539),
            Item::Rect(Rect {
                max_x: 283.0235,
                max_y: 100.434784,
                min_x: 282.56778,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(1540),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 104.154594,
                min_x: 282.56778,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(1541),
            Item::Rect(Rect {
                max_x: 289.08,
                max_y: 109.565216,
                min_x: 285.35294,
                min_y: 104.154594,
            }),
        ),
        (
            ItemId(1542),
            Item::Rect(Rect {
                max_x: 289.08,
                max_y: 118.695656,
                min_x: 288.38116,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1543),
            Item::Rect(Rect {
                max_x: 288.38116,
                max_y: 121.90365,
                min_x: 285.35294,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(1544),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 121.90365,
                min_x: 279.52942,
                min_y: 121.434784,
            }),
        ),
        (
            ItemId(1545),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 121.434784,
                min_x: 273.70587,
                min_y: 121.41011,
            }),
        ),
        (
            ItemId(1546),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 127.82609,
                min_x: 269.5,
                min_y: 121.41011,
            }),
        ),
        (
            ItemId(1547),
            Item::Rect(Rect {
                max_x: 269.5,
                max_y: 130.51152,
                min_x: 267.88235,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1548),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 136.95653,
                min_x: 264.0,
                min_y: 130.51152,
            }),
        ),
        (
            ItemId(1549),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 143.21739,
                min_x: 264.0,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1550),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 143.61928,
                min_x: 267.88235,
                min_y: 143.21739,
            }),
        ),
        (
            ItemId(1551),
            Item::Rect(Rect {
                max_x: 275.41867,
                max_y: 146.08696,
                min_x: 273.70587,
                min_y: 143.61928,
            }),
        ),
        (
            ItemId(1552),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 152.53198,
                min_x: 275.41867,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1553),
            Item::Rect(Rect {
                max_x: 283.76468,
                max_y: 152.53198,
                min_x: 279.52942,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1554),
            Item::Rect(Rect {
                max_x: 283.76468,
                max_y: 146.08696,
                min_x: 283.7353,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1555),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 136.95653,
                min_x: 283.7353,
                min_y: 134.2711,
            }),
        ),
        (
            ItemId(1556),
            Item::Rect(Rect {
                max_x: 289.13034,
                max_y: 134.2711,
                min_x: 285.35294,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1557),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 127.82609,
                min_x: 289.13034,
                min_y: 123.07826,
            }),
        ),
        (
            ItemId(1558),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 5.748792,
                min_x: 256.2353,
                min_y: 5.0374813,
            }),
        ),
        (
            ItemId(1559),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 5.748792,
                min_x: 252.4276,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1560),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 3.828892,
                min_x: 0.0,
                min_y: 3.2608697,
            }),
        ),
        (
            ItemId(1561),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 3.652174,
                min_x: 5.8235292,
                min_y: 3.2608697,
            }),
        ),
        (
            ItemId(1562),
            Item::Rect(Rect {
                max_x: 13.976471,
                max_y: 3.652174,
                min_x: 11.6470585,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1563),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 180.19182,
                min_x: 1.7794117,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1564),
            Item::Rect(Rect {
                max_x: 10.1055355,
                max_y: 180.19182,
                min_x: 5.8235292,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1565),
            Item::Rect(Rect {
                max_x: 10.1055355,
                max_y: 173.47827,
                min_x: 5.8235292,
                min_y: 167.13768,
            }),
        ),
        (
            ItemId(1566),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 173.47827,
                min_x: 1.7794117,
                min_y: 167.13768,
            }),
        ),
        (
            ItemId(1567),
            Item::Rect(Rect {
                max_x: 276.0784,
                max_y: 36.52174,
                min_x: 273.70587,
                min_y: 32.934784,
            }),
        ),
        (
            ItemId(1568),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 32.934784,
                min_x: 269.8982,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(1569),
            Item::Rect(Rect {
                max_x: 270.47058,
                max_y: 27.391304,
                min_x: 269.8982,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1570),
            Item::Rect(Rect {
                max_x: 270.47058,
                max_y: 18.26087,
                min_x: 267.88235,
                min_y: 14.608696,
            }),
        ),
        (
            ItemId(1571),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 14.608696,
                min_x: 264.50095,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1572),
            Item::Rect(Rect {
                max_x: 264.50095,
                max_y: 9.130435,
                min_x: 262.0588,
                min_y: 5.0374813,
            }),
        ),
        (
            ItemId(1573),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 50.568565,
                min_x: 282.09174,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(1574),
            Item::Rect(Rect {
                max_x: 282.09174,
                max_y: 45.652176,
                min_x: 279.52942,
                min_y: 41.93237,
            }),
        ),
        (
            ItemId(1575),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 41.93237,
                min_x: 276.0784,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(1576),
            Item::Rect(Rect {
                max_x: 113.17058,
                max_y: 113.39411,
                min_x: 110.64706,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1577),
            Item::Rect(Rect {
                max_x: 113.17058,
                max_y: 109.565216,
                min_x: 110.64706,
                min_y: 105.47227,
            }),
        ),
        (
            ItemId(1578),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 109.565216,
                min_x: 108.204926,
                min_y: 105.47227,
            }),
        ),
        (
            ItemId(1579),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 113.39411,
                min_x: 108.204926,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1580),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 164.34782,
                min_x: 286.94116,
                min_y: 157.70752,
            }),
        ),
        (
            ItemId(1581),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 170.98814,
                min_x: 286.94116,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(1582),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 59.347828,
                min_x: 288.14822,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(1583),
            Item::Rect(Rect {
                max_x: 288.14822,
                max_y: 54.782608,
                min_x: 285.35294,
                min_y: 50.568565,
            }),
        ),
        (
            ItemId(1584),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 6.425121,
                min_x: 251.97963,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1585),
            Item::Rect(Rect {
                max_x: 236.82353,
                max_y: 9.130435,
                min_x: 232.94116,
                min_y: 2.556522,
            }),
        ),
        (
            ItemId(1586),
            Item::Rect(Rect {
                max_x: 238.7647,
                max_y: 12.29097,
                min_x: 236.82353,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1587),
            Item::Rect(Rect {
                max_x: 242.30042,
                max_y: 18.26087,
                min_x: 238.7647,
                min_y: 12.29097,
            }),
        ),
        (
            ItemId(1588),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 22.278261,
                min_x: 242.30042,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1589),
            Item::Rect(Rect {
                max_x: 248.13298,
                max_y: 27.391304,
                min_x: 244.58823,
                min_y: 22.278261,
            }),
        ),
        (
            ItemId(1590),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 30.224888,
                min_x: 248.13298,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(1591),
            Item::Rect(Rect {
                max_x: 254.7255,
                max_y: 36.52174,
                min_x: 250.41176,
                min_y: 30.224888,
            }),
        ),
        (
            ItemId(1592),
            Item::Rect(Rect {
                max_x: 254.7255,
                max_y: 45.652176,
                min_x: 254.22719,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(1593),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 49.033817,
                min_x: 254.22719,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(1594),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 49.033817,
                min_x: 256.2353,
                min_y: 48.019325,
            }),
        ),
        (
            ItemId(1595),
            Item::Rect(Rect {
                max_x: 266.37256,
                max_y: 54.782608,
                min_x: 262.0588,
                min_y: 48.019325,
            }),
        ),
        (
            ItemId(1596),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 57.240803,
                min_x: 266.37256,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(1597),
            Item::Rect(Rect {
                max_x: 272.138,
                max_y: 63.913044,
                min_x: 267.88235,
                min_y: 57.240803,
            }),
        ),
        (
            ItemId(1598),
            Item::Rect(Rect {
                max_x: 272.138,
                max_y: 73.04348,
                min_x: 271.46603,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(1599),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 76.84783,
                min_x: 271.46603,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(1600),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 76.84783,
                min_x: 273.70587,
                min_y: 75.41063,
            }),
        ),
        (
            ItemId(1601),
            Item::Rect(Rect {
                max_x: 283.84314,
                max_y: 82.17391,
                min_x: 279.52942,
                min_y: 75.41063,
            }),
        ),
        (
            ItemId(1602),
            Item::Rect(Rect {
                max_x: 283.84314,
                max_y: 91.30435,
                min_x: 283.4894,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(1603),
            Item::Rect(Rect {
                max_x: 283.4894,
                max_y: 100.434784,
                min_x: 283.07416,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(1604),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 103.478264,
                min_x: 283.07416,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(1605),
            Item::Rect(Rect {
                max_x: 289.54587,
                max_y: 109.565216,
                min_x: 285.35294,
                min_y: 103.478264,
            }),
        ),
        (
            ItemId(1606),
            Item::Rect(Rect {
                max_x: 289.54587,
                max_y: 118.695656,
                min_x: 288.84705,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1607),
            Item::Rect(Rect {
                max_x: 288.84705,
                max_y: 122.39718,
                min_x: 285.35294,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(1608),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 122.39718,
                min_x: 279.52942,
                min_y: 121.89131,
            }),
        ),
        (
            ItemId(1609),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 121.90365,
                min_x: 273.70587,
                min_y: 121.89131,
            }),
        ),
        (
            ItemId(1610),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 127.82609,
                min_x: 269.82352,
                min_y: 121.90365,
            }),
        ),
        (
            ItemId(1611),
            Item::Rect(Rect {
                max_x: 269.82352,
                max_y: 131.0486,
                min_x: 267.88235,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1612),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 136.95653,
                min_x: 264.32352,
                min_y: 131.0486,
            }),
        ),
        (
            ItemId(1613),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 142.69566,
                min_x: 264.32352,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1614),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 143.12573,
                min_x: 267.88235,
                min_y: 142.69566,
            }),
        ),
        (
            ItemId(1615),
            Item::Rect(Rect {
                max_x: 275.76123,
                max_y: 146.08696,
                min_x: 273.70587,
                min_y: 143.12573,
            }),
        ),
        (
            ItemId(1616),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 151.99489,
                min_x: 275.76123,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1617),
            Item::Rect(Rect {
                max_x: 283.41177,
                max_y: 151.99489,
                min_x: 279.52942,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1618),
            Item::Rect(Rect {
                max_x: 283.41177,
                max_y: 146.08696,
                min_x: 283.41177,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1619),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 136.95653,
                min_x: 283.41177,
                min_y: 133.73401,
            }),
        ),
        (
            ItemId(1620),
            Item::Rect(Rect {
                max_x: 288.81555,
                max_y: 133.73401,
                min_x: 285.35294,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1621),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 127.82609,
                min_x: 288.81555,
                min_y: 122.347824,
            }),
        ),
        (
            ItemId(1622),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 3.0434785,
                min_x: 5.8235292,
                min_y: 2.6086957,
            }),
        ),
        (
            ItemId(1623),
            Item::Rect(Rect {
                max_x: 13.588235,
                max_y: 3.0434785,
                min_x: 11.6470585,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1624),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 3.2398317,
                min_x: 0.0,
                min_y: 2.6086957,
            }),
        ),
        (
            ItemId(1625),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 3.719807,
                min_x: 227.11765,
                min_y: 2.556522,
            }),
        ),
        (
            ItemId(1626),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 179.65472,
                min_x: 2.102941,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1627),
            Item::Rect(Rect {
                max_x: 9.762975,
                max_y: 179.65472,
                min_x: 5.8235292,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1628),
            Item::Rect(Rect {
                max_x: 9.762975,
                max_y: 173.47827,
                min_x: 5.8235292,
                min_y: 167.64493,
            }),
        ),
        (
            ItemId(1629),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 173.47827,
                min_x: 2.102941,
                min_y: 167.64493,
            }),
        ),
        (
            ItemId(1630),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 6.425121,
                min_x: 256.2353,
                min_y: 5.667166,
            }),
        ),
        (
            ItemId(1631),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 3.719807,
                min_x: 224.7451,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1632),
            Item::Rect(Rect {
                max_x: 270.03918,
                max_y: 27.391304,
                min_x: 269.45023,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1633),
            Item::Rect(Rect {
                max_x: 270.03918,
                max_y: 18.26087,
                min_x: 267.88235,
                min_y: 15.217392,
            }),
        ),
        (
            ItemId(1634),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 15.217392,
                min_x: 264.12524,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1635),
            Item::Rect(Rect {
                max_x: 264.12524,
                max_y: 9.130435,
                min_x: 262.0588,
                min_y: 5.667166,
            }),
        ),
        (
            ItemId(1636),
            Item::Rect(Rect {
                max_x: 281.6259,
                max_y: 45.652176,
                min_x: 279.52942,
                min_y: 42.608696,
            }),
        ),
        (
            ItemId(1637),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 42.608696,
                min_x: 275.64703,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(1638),
            Item::Rect(Rect {
                max_x: 275.64703,
                max_y: 36.52174,
                min_x: 273.70587,
                min_y: 33.586956,
            }),
        ),
        (
            ItemId(1639),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 33.586956,
                min_x: 269.45023,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(1640),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 60.050167,
                min_x: 287.68234,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(1641),
            Item::Rect(Rect {
                max_x: 287.68234,
                max_y: 54.782608,
                min_x: 285.35294,
                min_y: 51.270905,
            }),
        ),
        (
            ItemId(1642),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 51.270905,
                min_x: 281.6259,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(1643),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 112.805046,
                min_x: 108.580635,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1644),
            Item::Rect(Rect {
                max_x: 112.78235,
                max_y: 112.805046,
                min_x: 110.64706,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1645),
            Item::Rect(Rect {
                max_x: 112.78235,
                max_y: 109.565216,
                min_x: 110.64706,
                min_y: 106.10195,
            }),
        ),
        (
            ItemId(1646),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 109.565216,
                min_x: 108.580635,
                min_y: 106.10195,
            }),
        ),
        (
            ItemId(1647),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 170.43478,
                min_x: 287.2941,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(1648),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 164.34782,
                min_x: 287.2941,
                min_y: 158.26088,
            }),
        ),
        (
            ItemId(1649),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 179.11765,
                min_x: 2.4264705,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1650),
            Item::Rect(Rect {
                max_x: 9.420415,
                max_y: 179.11765,
                min_x: 5.8235292,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1651),
            Item::Rect(Rect {
                max_x: 9.420415,
                max_y: 173.47827,
                min_x: 5.8235292,
                min_y: 168.15218,
            }),
        ),
        (
            ItemId(1652),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 173.47827,
                min_x: 2.4264705,
                min_y: 168.15218,
            }),
        ),
        (
            ItemId(1653),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 7.1014495,
                min_x: 251.53166,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1654),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 3.0434785,
                min_x: 227.11765,
                min_y: 1.826087,
            }),
        ),
        (
            ItemId(1655),
            Item::Rect(Rect {
                max_x: 237.2549,
                max_y: 9.130435,
                min_x: 232.94116,
                min_y: 1.826087,
            }),
        ),
        (
            ItemId(1656),
            Item::Rect(Rect {
                max_x: 238.7647,
                max_y: 11.58863,
                min_x: 237.2549,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1657),
            Item::Rect(Rect {
                max_x: 242.71637,
                max_y: 18.26087,
                min_x: 238.7647,
                min_y: 11.58863,
            }),
        ),
        (
            ItemId(1658),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 21.547829,
                min_x: 242.71637,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1659),
            Item::Rect(Rect {
                max_x: 248.63937,
                max_y: 27.391304,
                min_x: 244.58823,
                min_y: 21.547829,
            }),
        ),
        (
            ItemId(1660),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 29.595203,
                min_x: 248.63937,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(1661),
            Item::Rect(Rect {
                max_x: 255.15686,
                max_y: 36.52174,
                min_x: 250.41176,
                min_y: 29.595203,
            }),
        ),
        (
            ItemId(1662),
            Item::Rect(Rect {
                max_x: 255.15686,
                max_y: 45.652176,
                min_x: 254.62878,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(1663),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 48.357487,
                min_x: 254.62878,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(1664),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 48.357487,
                min_x: 256.2353,
                min_y: 47.342995,
            }),
        ),
        (
            ItemId(1665),
            Item::Rect(Rect {
                max_x: 266.80392,
                max_y: 54.782608,
                min_x: 262.0588,
                min_y: 47.342995,
            }),
        ),
        (
            ItemId(1666),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 56.53846,
                min_x: 266.80392,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(1667),
            Item::Rect(Rect {
                max_x: 272.58597,
                max_y: 63.913044,
                min_x: 267.88235,
                min_y: 56.53846,
            }),
        ),
        (
            ItemId(1668),
            Item::Rect(Rect {
                max_x: 272.58597,
                max_y: 73.04348,
                min_x: 271.914,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(1669),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 76.08695,
                min_x: 271.914,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(1670),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 76.08695,
                min_x: 273.70587,
                min_y: 74.73431,
            }),
        ),
        (
            ItemId(1671),
            Item::Rect(Rect {
                max_x: 284.2745,
                max_y: 82.17391,
                min_x: 279.52942,
                min_y: 74.73431,
            }),
        ),
        (
            ItemId(1672),
            Item::Rect(Rect {
                max_x: 284.2745,
                max_y: 91.30435,
                min_x: 283.95526,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(1673),
            Item::Rect(Rect {
                max_x: 283.95526,
                max_y: 100.434784,
                min_x: 283.58057,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(1674),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 102.80193,
                min_x: 283.58057,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(1675),
            Item::Rect(Rect {
                max_x: 290.01175,
                max_y: 109.565216,
                min_x: 285.35294,
                min_y: 102.80193,
            }),
        ),
        (
            ItemId(1676),
            Item::Rect(Rect {
                max_x: 290.01175,
                max_y: 118.695656,
                min_x: 289.31293,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1677),
            Item::Rect(Rect {
                max_x: 289.31293,
                max_y: 122.89072,
                min_x: 285.35294,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(1678),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 122.89072,
                min_x: 279.52942,
                min_y: 122.347824,
            }),
        ),
        (
            ItemId(1679),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 122.39718,
                min_x: 273.70587,
                min_y: 122.347824,
            }),
        ),
        (
            ItemId(1680),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 127.82609,
                min_x: 270.14706,
                min_y: 122.39718,
            }),
        ),
        (
            ItemId(1681),
            Item::Rect(Rect {
                max_x: 270.14706,
                max_y: 131.58568,
                min_x: 267.88235,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1682),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 136.95653,
                min_x: 264.64703,
                min_y: 131.58568,
            }),
        ),
        (
            ItemId(1683),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 142.17392,
                min_x: 264.64703,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1684),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 142.6322,
                min_x: 267.88235,
                min_y: 142.17392,
            }),
        ),
        (
            ItemId(1685),
            Item::Rect(Rect {
                max_x: 276.10382,
                max_y: 146.08696,
                min_x: 273.70587,
                min_y: 142.6322,
            }),
        ),
        (
            ItemId(1686),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 151.45781,
                min_x: 276.10382,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1687),
            Item::Rect(Rect {
                max_x: 283.0588,
                max_y: 151.45781,
                min_x: 279.52942,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1688),
            Item::Rect(Rect {
                max_x: 283.08823,
                max_y: 146.08696,
                min_x: 283.0588,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1689),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 136.95653,
                min_x: 283.08823,
                min_y: 133.19693,
            }),
        ),
        (
            ItemId(1690),
            Item::Rect(Rect {
                max_x: 288.50076,
                max_y: 133.19693,
                min_x: 285.35294,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1691),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 127.82609,
                min_x: 288.50076,
                min_y: 121.61739,
            }),
        ),
        (
            ItemId(1692),
            Item::Rect(Rect {
                max_x: 13.2,
                max_y: 2.4347827,
                min_x: 11.6470585,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1693),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 2.6507714,
                min_x: 0.0,
                min_y: 1.9565217,
            }),
        ),
        (
            ItemId(1694),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 2.4347827,
                min_x: 5.8235292,
                min_y: 1.9565217,
            }),
        ),
        (
            ItemId(1695),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 7.1014495,
                min_x: 256.2353,
                min_y: 6.2968516,
            }),
        ),
        (
            ItemId(1696),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 3.0434785,
                min_x: 225.17647,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1697),
            Item::Rect(Rect {
                max_x: 112.39411,
                max_y: 112.21599,
                min_x: 110.64706,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1698),
            Item::Rect(Rect {
                max_x: 112.39411,
                max_y: 109.565216,
                min_x: 110.64706,
                min_y: 106.731636,
            }),
        ),
        (
            ItemId(1699),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 109.565216,
                min_x: 108.95635,
                min_y: 106.731636,
            }),
        ),
        (
            ItemId(1700),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 112.21599,
                min_x: 108.95635,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1701),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 169.88142,
                min_x: 287.64706,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(1702),
            Item::Rect(Rect {
                max_x: 263.7495,
                max_y: 9.130435,
                min_x: 262.0588,
                min_y: 6.2968516,
            }),
        ),
        (
            ItemId(1703),
            Item::Rect(Rect {
                max_x: 275.21567,
                max_y: 36.52174,
                min_x: 273.70587,
                min_y: 34.239132,
            }),
        ),
        (
            ItemId(1704),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 34.239132,
                min_x: 269.00226,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(1705),
            Item::Rect(Rect {
                max_x: 269.60782,
                max_y: 27.391304,
                min_x: 269.00226,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1706),
            Item::Rect(Rect {
                max_x: 269.60782,
                max_y: 18.26087,
                min_x: 267.88235,
                min_y: 15.826087,
            }),
        ),
        (
            ItemId(1707),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 15.826087,
                min_x: 263.7495,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1708),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 51.973244,
                min_x: 281.15997,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(1709),
            Item::Rect(Rect {
                max_x: 281.15997,
                max_y: 45.652176,
                min_x: 279.52942,
                min_y: 43.285027,
            }),
        ),
        (
            ItemId(1710),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 43.285027,
                min_x: 275.21567,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(1711),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 164.34782,
                min_x: 287.64706,
                min_y: 158.81424,
            }),
        ),
        (
            ItemId(1712),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 60.75251,
                min_x: 287.21646,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(1713),
            Item::Rect(Rect {
                max_x: 287.21646,
                max_y: 54.782608,
                min_x: 285.35294,
                min_y: 51.973244,
            }),
        ),
        (
            ItemId(1714),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 2.3671498,
                min_x: 227.11765,
                min_y: 1.0956522,
            }),
        ),
        (
            ItemId(1715),
            Item::Rect(Rect {
                max_x: 237.68628,
                max_y: 9.130435,
                min_x: 232.94116,
                min_y: 1.0956522,
            }),
        ),
        (
            ItemId(1716),
            Item::Rect(Rect {
                max_x: 238.7647,
                max_y: 10.886288,
                min_x: 237.68628,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1717),
            Item::Rect(Rect {
                max_x: 243.13234,
                max_y: 18.26087,
                min_x: 238.7647,
                min_y: 10.886288,
            }),
        ),
        (
            ItemId(1718),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 20.817392,
                min_x: 243.13234,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1719),
            Item::Rect(Rect {
                max_x: 249.14577,
                max_y: 27.391304,
                min_x: 244.58823,
                min_y: 20.817392,
            }),
        ),
        (
            ItemId(1720),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 28.965519,
                min_x: 249.14577,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(1721),
            Item::Rect(Rect {
                max_x: 255.58823,
                max_y: 36.52174,
                min_x: 250.41176,
                min_y: 28.965519,
            }),
        ),
        (
            ItemId(1722),
            Item::Rect(Rect {
                max_x: 255.58823,
                max_y: 45.652176,
                min_x: 255.03041,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(1723),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 47.68116,
                min_x: 255.03041,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(1724),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 47.68116,
                min_x: 256.2353,
                min_y: 46.666668,
            }),
        ),
        (
            ItemId(1725),
            Item::Rect(Rect {
                max_x: 267.2353,
                max_y: 54.782608,
                min_x: 262.0588,
                min_y: 46.666668,
            }),
        ),
        (
            ItemId(1726),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 55.83612,
                min_x: 267.2353,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(1727),
            Item::Rect(Rect {
                max_x: 273.03394,
                max_y: 63.913044,
                min_x: 267.88235,
                min_y: 55.83612,
            }),
        ),
        (
            ItemId(1728),
            Item::Rect(Rect {
                max_x: 273.03394,
                max_y: 73.04348,
                min_x: 272.36197,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(1729),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 75.32609,
                min_x: 272.36197,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(1730),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 75.32609,
                min_x: 273.70587,
                min_y: 74.05797,
            }),
        ),
        (
            ItemId(1731),
            Item::Rect(Rect {
                max_x: 284.70587,
                max_y: 82.17391,
                min_x: 279.52942,
                min_y: 74.05797,
            }),
        ),
        (
            ItemId(1732),
            Item::Rect(Rect {
                max_x: 284.70587,
                max_y: 91.30435,
                min_x: 284.42117,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(1733),
            Item::Rect(Rect {
                max_x: 284.42117,
                max_y: 100.434784,
                min_x: 284.08694,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(1734),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 102.12561,
                min_x: 284.08694,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(1735),
            Item::Rect(Rect {
                max_x: 290.47763,
                max_y: 109.565216,
                min_x: 285.35294,
                min_y: 102.12561,
            }),
        ),
        (
            ItemId(1736),
            Item::Rect(Rect {
                max_x: 290.47763,
                max_y: 118.695656,
                min_x: 289.7788,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1737),
            Item::Rect(Rect {
                max_x: 289.7788,
                max_y: 123.384254,
                min_x: 285.35294,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(1738),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 123.384254,
                min_x: 279.52942,
                min_y: 122.80435,
            }),
        ),
        (
            ItemId(1739),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 122.89072,
                min_x: 273.70587,
                min_y: 122.80435,
            }),
        ),
        (
            ItemId(1740),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 127.82609,
                min_x: 270.47058,
                min_y: 122.89072,
            }),
        ),
        (
            ItemId(1741),
            Item::Rect(Rect {
                max_x: 270.47058,
                max_y: 132.12277,
                min_x: 267.88235,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1742),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 136.95653,
                min_x: 264.97058,
                min_y: 132.12277,
            }),
        ),
        (
            ItemId(1743),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 141.65218,
                min_x: 264.97058,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1744),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 142.13867,
                min_x: 267.88235,
                min_y: 141.65218,
            }),
        ),
        (
            ItemId(1745),
            Item::Rect(Rect {
                max_x: 276.44635,
                max_y: 146.08696,
                min_x: 273.70587,
                min_y: 142.13867,
            }),
        ),
        (
            ItemId(1746),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 150.92072,
                min_x: 276.44635,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1747),
            Item::Rect(Rect {
                max_x: 282.70587,
                max_y: 150.92072,
                min_x: 279.52942,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1748),
            Item::Rect(Rect {
                max_x: 282.7647,
                max_y: 146.08696,
                min_x: 282.70587,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1749),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 136.95653,
                min_x: 282.7647,
                min_y: 132.65985,
            }),
        ),
        (
            ItemId(1750),
            Item::Rect(Rect {
                max_x: 288.186,
                max_y: 132.65985,
                min_x: 285.35294,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1751),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 127.82609,
                min_x: 288.186,
                min_y: 120.886955,
            }),
        ),
        (
            ItemId(1752),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 7.777778,
                min_x: 251.0837,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1753),
            Item::Rect(Rect {
                max_x: 12.811765,
                max_y: 1.826087,
                min_x: 11.6470585,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1754),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 1.826087,
                min_x: 5.8235292,
                min_y: 1.3043479,
            }),
        ),
        (
            ItemId(1755),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 7.777778,
                min_x: 256.2353,
                min_y: 6.9265366,
            }),
        ),
        (
            ItemId(1756),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 2.3671498,
                min_x: 225.60785,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1757),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 178.58055,
                min_x: 2.7499998,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1758),
            Item::Rect(Rect {
                max_x: 9.077855,
                max_y: 178.58055,
                min_x: 5.8235292,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1759),
            Item::Rect(Rect {
                max_x: 9.077855,
                max_y: 173.47827,
                min_x: 5.8235292,
                min_y: 168.65942,
            }),
        ),
        (
            ItemId(1760),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 173.47827,
                min_x: 2.7499998,
                min_y: 168.65942,
            }),
        ),
        (
            ItemId(1761),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 2.061711,
                min_x: 0.0,
                min_y: 1.3043479,
            }),
        ),
        (
            ItemId(1762),
            Item::Rect(Rect {
                max_x: 280.69412,
                max_y: 45.652176,
                min_x: 279.52942,
                min_y: 43.961353,
            }),
        ),
        (
            ItemId(1763),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 43.961353,
                min_x: 274.7843,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(1764),
            Item::Rect(Rect {
                max_x: 274.7843,
                max_y: 36.52174,
                min_x: 273.70587,
                min_y: 34.891304,
            }),
        ),
        (
            ItemId(1765),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 34.891304,
                min_x: 268.5543,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(1766),
            Item::Rect(Rect {
                max_x: 269.17645,
                max_y: 27.391304,
                min_x: 268.5543,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1767),
            Item::Rect(Rect {
                max_x: 269.17645,
                max_y: 18.26087,
                min_x: 267.88235,
                min_y: 16.434782,
            }),
        ),
        (
            ItemId(1768),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 16.434782,
                min_x: 263.3738,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1769),
            Item::Rect(Rect {
                max_x: 263.3738,
                max_y: 9.130435,
                min_x: 262.0588,
                min_y: 6.9265366,
            }),
        ),
        (
            ItemId(1770),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 61.45485,
                min_x: 286.75058,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(1771),
            Item::Rect(Rect {
                max_x: 286.75058,
                max_y: 54.782608,
                min_x: 285.35294,
                min_y: 52.675587,
            }),
        ),
        (
            ItemId(1772),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 52.675587,
                min_x: 280.69412,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(1773),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 111.62693,
                min_x: 109.33206,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1774),
            Item::Rect(Rect {
                max_x: 112.00588,
                max_y: 111.62693,
                min_x: 110.64706,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1775),
            Item::Rect(Rect {
                max_x: 112.00588,
                max_y: 109.565216,
                min_x: 110.64706,
                min_y: 107.36132,
            }),
        ),
        (
            ItemId(1776),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 109.565216,
                min_x: 109.33206,
                min_y: 107.36132,
            }),
        ),
        (
            ItemId(1777),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 164.34782,
                min_x: 287.99997,
                min_y: 159.3676,
            }),
        ),
        (
            ItemId(1778),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 169.32806,
                min_x: 287.99997,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(1779),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 178.04349,
                min_x: 3.0735295,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1780),
            Item::Rect(Rect {
                max_x: 8.735294,
                max_y: 178.04349,
                min_x: 5.8235292,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1781),
            Item::Rect(Rect {
                max_x: 8.735294,
                max_y: 173.47827,
                min_x: 5.8235292,
                min_y: 169.16667,
            }),
        ),
        (
            ItemId(1782),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 173.47827,
                min_x: 3.0735295,
                min_y: 169.16667,
            }),
        ),
        (
            ItemId(1783),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 8.454106,
                min_x: 250.63573,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1784),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 1.4726508,
                min_x: 0.0,
                min_y: 0.65217394,
            }),
        ),
        (
            ItemId(1785),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 1.2173914,
                min_x: 5.8235292,
                min_y: 0.65217394,
            }),
        ),
        (
            ItemId(1786),
            Item::Rect(Rect {
                max_x: 12.42353,
                max_y: 1.2173914,
                min_x: 11.6470585,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1787),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 9.130435,
                min_x: 244.58823,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1788),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 9.130435,
                min_x: 250.41176,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1789),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 9.130435,
                min_x: 250.41176,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1790),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 9.130435,
                min_x: 244.58823,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1791),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 9.130435,
                min_x: 238.7647,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1792),
            Item::Rect(Rect {
                max_x: 238.7647,
                max_y: 0.0,
                min_x: 238.7647,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1793),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 1.6908213,
                min_x: 226.03922,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1794),
            Item::Rect(Rect {
                max_x: 232.94116,
                max_y: 1.6908214,
                min_x: 227.11765,
                min_y: 0.3652174,
            }),
        ),
        (
            ItemId(1795),
            Item::Rect(Rect {
                max_x: 238.11765,
                max_y: 9.130435,
                min_x: 232.94116,
                min_y: 0.3652174,
            }),
        ),
        (
            ItemId(1796),
            Item::Rect(Rect {
                max_x: 238.7647,
                max_y: 10.183947,
                min_x: 238.11765,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1797),
            Item::Rect(Rect {
                max_x: 243.54832,
                max_y: 18.26087,
                min_x: 238.7647,
                min_y: 10.183947,
            }),
        ),
        (
            ItemId(1798),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 20.086958,
                min_x: 243.54832,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1799),
            Item::Rect(Rect {
                max_x: 249.65216,
                max_y: 27.391304,
                min_x: 244.58823,
                min_y: 20.086958,
            }),
        ),
        (
            ItemId(1800),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 28.335835,
                min_x: 249.65216,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(1801),
            Item::Rect(Rect {
                max_x: 256.0196,
                max_y: 36.52174,
                min_x: 250.41176,
                min_y: 28.335835,
            }),
        ),
        (
            ItemId(1802),
            Item::Rect(Rect {
                max_x: 256.0196,
                max_y: 45.652176,
                min_x: 255.43204,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(1803),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 47.00483,
                min_x: 255.43204,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(1804),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 47.00483,
                min_x: 256.2353,
                min_y: 45.990337,
            }),
        ),
        (
            ItemId(1805),
            Item::Rect(Rect {
                max_x: 267.66666,
                max_y: 54.782608,
                min_x: 262.0588,
                min_y: 45.990337,
            }),
        ),
        (
            ItemId(1806),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 55.13378,
                min_x: 267.66666,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(1807),
            Item::Rect(Rect {
                max_x: 273.4819,
                max_y: 63.913044,
                min_x: 267.88235,
                min_y: 55.13378,
            }),
        ),
        (
            ItemId(1808),
            Item::Rect(Rect {
                max_x: 273.4819,
                max_y: 73.04348,
                min_x: 272.80994,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(1809),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 74.56522,
                min_x: 272.80994,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(1810),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 74.56522,
                min_x: 273.70587,
                min_y: 73.381645,
            }),
        ),
        (
            ItemId(1811),
            Item::Rect(Rect {
                max_x: 285.13724,
                max_y: 82.17391,
                min_x: 279.52942,
                min_y: 73.381645,
            }),
        ),
        (
            ItemId(1812),
            Item::Rect(Rect {
                max_x: 285.13724,
                max_y: 91.30435,
                min_x: 284.88705,
                min_y: 82.17391,
            }),
        ),
        (
            ItemId(1813),
            Item::Rect(Rect {
                max_x: 284.88705,
                max_y: 100.434784,
                min_x: 284.59332,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(1814),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 101.44927,
                min_x: 284.59332,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(1815),
            Item::Rect(Rect {
                max_x: 290.9435,
                max_y: 109.565216,
                min_x: 285.35294,
                min_y: 101.44927,
            }),
        ),
        (
            ItemId(1816),
            Item::Rect(Rect {
                max_x: 290.9435,
                max_y: 118.695656,
                min_x: 290.2447,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1817),
            Item::Rect(Rect {
                max_x: 290.2447,
                max_y: 123.87779,
                min_x: 285.35294,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(1818),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 123.87779,
                min_x: 279.52942,
                min_y: 123.26087,
            }),
        ),
        (
            ItemId(1819),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 123.384254,
                min_x: 273.70587,
                min_y: 123.26087,
            }),
        ),
        (
            ItemId(1820),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 127.82609,
                min_x: 270.7941,
                min_y: 123.384254,
            }),
        ),
        (
            ItemId(1821),
            Item::Rect(Rect {
                max_x: 270.7941,
                max_y: 132.65985,
                min_x: 267.88235,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1822),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 136.95653,
                min_x: 265.29413,
                min_y: 132.65985,
            }),
        ),
        (
            ItemId(1823),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 141.13043,
                min_x: 265.29413,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1824),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 141.64513,
                min_x: 267.88235,
                min_y: 141.13043,
            }),
        ),
        (
            ItemId(1825),
            Item::Rect(Rect {
                max_x: 276.7889,
                max_y: 146.08696,
                min_x: 273.70587,
                min_y: 141.64513,
            }),
        ),
        (
            ItemId(1826),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 150.38364,
                min_x: 276.7889,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1827),
            Item::Rect(Rect {
                max_x: 282.35294,
                max_y: 150.38364,
                min_x: 279.52942,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1828),
            Item::Rect(Rect {
                max_x: 282.44116,
                max_y: 146.08696,
                min_x: 282.35294,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1829),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 136.95653,
                min_x: 282.44116,
                min_y: 132.12277,
            }),
        ),
        (
            ItemId(1830),
            Item::Rect(Rect {
                max_x: 287.87122,
                max_y: 132.12277,
                min_x: 285.35294,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1831),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 127.82609,
                min_x: 287.87122,
                min_y: 120.156525,
            }),
        ),
        (
            ItemId(1832),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 8.454106,
                min_x: 256.2353,
                min_y: 7.5562224,
            }),
        ),
        (
            ItemId(1833),
            Item::Rect(Rect {
                max_x: 274.35294,
                max_y: 36.52174,
                min_x: 273.70587,
                min_y: 35.54348,
            }),
        ),
        (
            ItemId(1834),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 35.54348,
                min_x: 268.10632,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(1835),
            Item::Rect(Rect {
                max_x: 268.7451,
                max_y: 27.391304,
                min_x: 268.10632,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1836),
            Item::Rect(Rect {
                max_x: 268.7451,
                max_y: 18.26087,
                min_x: 267.88235,
                min_y: 17.043478,
            }),
        ),
        (
            ItemId(1837),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 17.043478,
                min_x: 262.99808,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1838),
            Item::Rect(Rect {
                max_x: 262.99808,
                max_y: 9.130435,
                min_x: 262.0588,
                min_y: 7.5562224,
            }),
        ),
        (
            ItemId(1839),
            Item::Rect(Rect {
                max_x: 286.2847,
                max_y: 54.782608,
                min_x: 285.35294,
                min_y: 53.377926,
            }),
        ),
        (
            ItemId(1840),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 53.377926,
                min_x: 280.2282,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(1841),
            Item::Rect(Rect {
                max_x: 280.2282,
                max_y: 45.652176,
                min_x: 279.52942,
                min_y: 44.63768,
            }),
        ),
        (
            ItemId(1842),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 44.63768,
                min_x: 274.35294,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(1843),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 111.03787,
                min_x: 109.70777,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1844),
            Item::Rect(Rect {
                max_x: 111.61764,
                max_y: 111.03787,
                min_x: 110.64706,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1845),
            Item::Rect(Rect {
                max_x: 111.61764,
                max_y: 109.565216,
                min_x: 110.64706,
                min_y: 107.991005,
            }),
        ),
        (
            ItemId(1846),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 109.565216,
                min_x: 109.70777,
                min_y: 107.991005,
            }),
        ),
        (
            ItemId(1847),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 62.157192,
                min_x: 286.2847,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(1848),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 168.7747,
                min_x: 288.35294,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(1849),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 164.34782,
                min_x: 288.35294,
                min_y: 159.92096,
            }),
        ),
        (
            ItemId(1850),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 9.130435,
                min_x: 244.58823,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1851),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 1.0144928,
                min_x: 226.47058,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1852),
            Item::Rect(Rect {
                max_x: 231.48529,
                max_y: 1.0144928,
                min_x: 227.11765,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1853),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 177.50641,
                min_x: 3.3970587,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1854),
            Item::Rect(Rect {
                max_x: 8.392733,
                max_y: 177.50641,
                min_x: 5.8235292,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1855),
            Item::Rect(Rect {
                max_x: 8.392733,
                max_y: 173.47827,
                min_x: 5.8235292,
                min_y: 169.67392,
            }),
        ),
        (
            ItemId(1856),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 173.47827,
                min_x: 3.3970587,
                min_y: 169.67392,
            }),
        ),
        (
            ItemId(1857),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 9.130435,
                min_x: 256.2353,
                min_y: 8.185907,
            }),
        ),
        (
            ItemId(1858),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 15.978261,
                min_x: 256.2353,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1859),
            Item::Rect(Rect {
                max_x: 264.0,
                max_y: 18.26087,
                min_x: 262.0588,
                min_y: 15.978261,
            }),
        ),
        (
            ItemId(1860),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 24.347828,
                min_x: 264.0,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1861),
            Item::Rect(Rect {
                max_x: 268.31372,
                max_y: 24.347828,
                min_x: 267.88235,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1862),
            Item::Rect(Rect {
                max_x: 268.31372,
                max_y: 18.26087,
                min_x: 267.88235,
                min_y: 17.652174,
            }),
        ),
        (
            ItemId(1863),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 17.652174,
                min_x: 262.62238,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1864),
            Item::Rect(Rect {
                max_x: 262.62238,
                max_y: 9.130435,
                min_x: 262.0588,
                min_y: 8.185907,
            }),
        ),
        (
            ItemId(1865),
            Item::Rect(Rect {
                max_x: 12.035294,
                max_y: 0.6086957,
                min_x: 11.6470585,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1866),
            Item::Rect(Rect {
                max_x: 11.6470585,
                max_y: 0.6086957,
                min_x: 5.8235292,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1867),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 0.88359046,
                min_x: 0.0,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1868),
            Item::Rect(Rect {
                max_x: 238.7647,
                max_y: 9.130435,
                min_x: 238.54901,
                min_y: 6.086957,
            }),
        ),
        (
            ItemId(1869),
            Item::Rect(Rect {
                max_x: 238.7647,
                max_y: 9.481606,
                min_x: 238.54901,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1870),
            Item::Rect(Rect {
                max_x: 243.96428,
                max_y: 18.26087,
                min_x: 238.7647,
                min_y: 9.481606,
            }),
        ),
        (
            ItemId(1871),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 19.356522,
                min_x: 243.96428,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1872),
            Item::Rect(Rect {
                max_x: 250.15855,
                max_y: 27.391304,
                min_x: 244.58823,
                min_y: 19.356522,
            }),
        ),
        (
            ItemId(1873),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 27.706146,
                min_x: 250.15855,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(1874),
            Item::Rect(Rect {
                max_x: 253.32352,
                max_y: 27.706146,
                min_x: 250.41176,
                min_y: 27.391304,
            }),
        ),
        (
            ItemId(1875),
            Item::Rect(Rect {
                max_x: 253.32352,
                max_y: 27.391304,
                min_x: 250.41176,
                min_y: 22.826088,
            }),
        ),
        (
            ItemId(1876),
            Item::Rect(Rect {
                max_x: 250.41176,
                max_y: 22.826088,
                min_x: 248.95587,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1877),
            Item::Rect(Rect {
                max_x: 248.95587,
                max_y: 18.26087,
                min_x: 244.58823,
                min_y: 12.782609,
            }),
        ),
        (
            ItemId(1878),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 12.782609,
                min_x: 240.70587,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1879),
            Item::Rect(Rect {
                max_x: 240.70587,
                max_y: 9.130435,
                min_x: 238.7647,
                min_y: 6.086957,
            }),
        ),
        (
            ItemId(1880),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 100.434784,
                min_x: 285.09973,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(1881),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 100.77295,
                min_x: 285.09973,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(1882),
            Item::Rect(Rect {
                max_x: 288.2647,
                max_y: 100.77295,
                min_x: 285.35294,
                min_y: 100.434784,
            }),
        ),
        (
            ItemId(1883),
            Item::Rect(Rect {
                max_x: 288.2647,
                max_y: 100.434784,
                min_x: 285.35294,
                min_y: 91.30435,
            }),
        ),
        (
            ItemId(1884),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 123.87779,
                min_x: 273.70587,
                min_y: 123.7174,
            }),
        ),
        (
            ItemId(1885),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 127.82609,
                min_x: 271.11765,
                min_y: 123.87779,
            }),
        ),
        (
            ItemId(1886),
            Item::Rect(Rect {
                max_x: 271.11765,
                max_y: 133.19693,
                min_x: 267.88235,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1887),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 136.95653,
                min_x: 265.61765,
                min_y: 133.19693,
            }),
        ),
        (
            ItemId(1888),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 140.60869,
                min_x: 265.61765,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1889),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 141.15158,
                min_x: 267.88235,
                min_y: 140.60869,
            }),
        ),
        (
            ItemId(1890),
            Item::Rect(Rect {
                max_x: 277.13147,
                max_y: 146.08696,
                min_x: 273.70587,
                min_y: 141.15158,
            }),
        ),
        (
            ItemId(1891),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 149.84654,
                min_x: 277.13147,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1892),
            Item::Rect(Rect {
                max_x: 282.0,
                max_y: 149.84654,
                min_x: 279.52942,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1893),
            Item::Rect(Rect {
                max_x: 282.1176,
                max_y: 146.08696,
                min_x: 282.0,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1894),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 136.95653,
                min_x: 282.1176,
                min_y: 131.58568,
            }),
        ),
        (
            ItemId(1895),
            Item::Rect(Rect {
                max_x: 287.55643,
                max_y: 131.58568,
                min_x: 285.35294,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1896),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 127.82609,
                min_x: 287.55643,
                min_y: 119.42609,
            }),
        ),
        (
            ItemId(1897),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 36.52174,
                min_x: 270.7941,
                min_y: 36.195656,
            }),
        ),
        (
            ItemId(1898),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 41.086956,
                min_x: 270.7941,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(1899),
            Item::Rect(Rect {
                max_x: 276.61765,
                max_y: 45.652176,
                min_x: 273.70587,
                min_y: 41.086956,
            }),
        ),
        (
            ItemId(1900),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 50.217392,
                min_x: 276.61765,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(1901),
            Item::Rect(Rect {
                max_x: 281.47058,
                max_y: 54.782608,
                min_x: 279.52942,
                min_y: 50.217392,
            }),
        ),
        (
            ItemId(1902),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 60.869564,
                min_x: 281.47058,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(1903),
            Item::Rect(Rect {
                max_x: 286.8088,
                max_y: 63.913044,
                min_x: 285.35294,
                min_y: 60.869564,
            }),
        ),
        (
            ItemId(1904),
            Item::Rect(Rect {
                max_x: 288.2647,
                max_y: 73.04348,
                min_x: 286.8088,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(1905),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 77.608696,
                min_x: 288.2647,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(1906),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 73.804344,
                min_x: 273.2579,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(1907),
            Item::Rect(Rect {
                max_x: 277.58823,
                max_y: 73.804344,
                min_x: 273.70587,
                min_y: 73.04348,
            }),
        ),
        (
            ItemId(1908),
            Item::Rect(Rect {
                max_x: 277.58823,
                max_y: 73.04348,
                min_x: 273.70587,
                min_y: 66.95653,
            }),
        ),
        (
            ItemId(1909),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 73.04348,
                min_x: 273.2579,
                min_y: 66.95653,
            }),
        ),
        (
            ItemId(1910),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 110.44881,
                min_x: 110.08348,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1911),
            Item::Rect(Rect {
                max_x: 111.22941,
                max_y: 110.44881,
                min_x: 110.64706,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1912),
            Item::Rect(Rect {
                max_x: 111.22941,
                max_y: 109.565216,
                min_x: 110.64706,
                min_y: 108.6207,
            }),
        ),
        (
            ItemId(1913),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 109.565216,
                min_x: 110.08348,
                min_y: 108.6207,
            }),
        ),
        (
            ItemId(1914),
            Item::Rect(Rect {
                max_x: 285.81882,
                max_y: 54.782608,
                min_x: 285.35294,
                min_y: 54.08027,
            }),
        ),
        (
            ItemId(1915),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 54.08027,
                min_x: 279.76236,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(1916),
            Item::Rect(Rect {
                max_x: 279.76236,
                max_y: 45.652176,
                min_x: 279.52942,
                min_y: 45.31401,
            }),
        ),
        (
            ItemId(1917),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 45.31401,
                min_x: 273.92157,
                min_y: 36.52174,
            }),
        ),
        (
            ItemId(1918),
            Item::Rect(Rect {
                max_x: 273.92157,
                max_y: 36.52174,
                min_x: 273.70587,
                min_y: 36.195656,
            }),
        ),
        (
            ItemId(1919),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 118.695656,
                min_x: 290.71057,
                min_y: 112.608696,
            }),
        ),
        (
            ItemId(1920),
            Item::Rect(Rect {
                max_x: 290.71057,
                max_y: 124.37134,
                min_x: 285.35294,
                min_y: 118.695656,
            }),
        ),
        (
            ItemId(1921),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 124.37134,
                min_x: 279.52942,
                min_y: 123.7174,
            }),
        ),
        (
            ItemId(1922),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 46.328506,
                min_x: 255.83366,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(1923),
            Item::Rect(Rect {
                max_x: 260.11765,
                max_y: 46.328506,
                min_x: 256.2353,
                min_y: 45.652176,
            }),
        ),
        (
            ItemId(1924),
            Item::Rect(Rect {
                max_x: 260.11765,
                max_y: 45.652176,
                min_x: 256.2353,
                min_y: 39.56522,
            }),
        ),
        (
            ItemId(1925),
            Item::Rect(Rect {
                max_x: 256.2353,
                max_y: 45.652176,
                min_x: 255.83366,
                min_y: 39.56522,
            }),
        ),
        (
            ItemId(1926),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 164.34782,
                min_x: 288.70587,
                min_y: 160.47432,
            }),
        ),
        (
            ItemId(1927),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 168.22134,
                min_x: 288.70587,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(1928),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 62.859535,
                min_x: 285.81882,
                min_y: 54.782608,
            }),
        ),
        (
            ItemId(1929),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 9.130435,
                min_x: 260.11765,
                min_y: 8.815592,
            }),
        ),
        (
            ItemId(1930),
            Item::Rect(Rect {
                max_x: 262.0588,
                max_y: 11.413044,
                min_x: 260.11765,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1931),
            Item::Rect(Rect {
                max_x: 262.24667,
                max_y: 11.413044,
                min_x: 262.0588,
                min_y: 9.130435,
            }),
        ),
        (
            ItemId(1932),
            Item::Rect(Rect {
                max_x: 262.24667,
                max_y: 9.130435,
                min_x: 262.0588,
                min_y: 8.815592,
            }),
        ),
        (
            ItemId(1933),
            Item::Rect(Rect {
                max_x: 1.9411764,
                max_y: 0.29453015,
                min_x: 0.0,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1934),
            Item::Rect(Rect {
                max_x: 228.57352,
                max_y: 0.33816427,
                min_x: 227.11765,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1935),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 176.96931,
                min_x: 3.7205882,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1936),
            Item::Rect(Rect {
                max_x: 8.050173,
                max_y: 176.96931,
                min_x: 5.8235292,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1937),
            Item::Rect(Rect {
                max_x: 8.050173,
                max_y: 173.47827,
                min_x: 5.8235292,
                min_y: 170.18117,
            }),
        ),
        (
            ItemId(1938),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 173.47827,
                min_x: 3.7205882,
                min_y: 170.18117,
            }),
        ),
        (
            ItemId(1939),
            Item::Rect(Rect {
                max_x: 227.11765,
                max_y: 0.33816427,
                min_x: 226.90196,
                min_y: 0.0,
            }),
        ),
        (
            ItemId(1940),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 124.37134,
                min_x: 273.70587,
                min_y: 124.17392,
            }),
        ),
        (
            ItemId(1941),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 127.82609,
                min_x: 271.44116,
                min_y: 124.37134,
            }),
        ),
        (
            ItemId(1942),
            Item::Rect(Rect {
                max_x: 271.44116,
                max_y: 133.73401,
                min_x: 267.88235,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1943),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 136.95653,
                min_x: 265.94116,
                min_y: 133.73401,
            }),
        ),
        (
            ItemId(1944),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 140.08696,
                min_x: 265.94116,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1945),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 140.65805,
                min_x: 267.88235,
                min_y: 140.08696,
            }),
        ),
        (
            ItemId(1946),
            Item::Rect(Rect {
                max_x: 277.47406,
                max_y: 146.08696,
                min_x: 273.70587,
                min_y: 140.65805,
            }),
        ),
        (
            ItemId(1947),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 149.30946,
                min_x: 277.47406,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1948),
            Item::Rect(Rect {
                max_x: 281.64703,
                max_y: 149.30946,
                min_x: 279.52942,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1949),
            Item::Rect(Rect {
                max_x: 281.7941,
                max_y: 146.08696,
                min_x: 281.64703,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1950),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 136.95653,
                min_x: 281.7941,
                min_y: 131.0486,
            }),
        ),
        (
            ItemId(1951),
            Item::Rect(Rect {
                max_x: 287.24164,
                max_y: 131.0486,
                min_x: 285.35294,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1952),
            Item::Rect(Rect {
                max_x: 287.24164,
                max_y: 127.82609,
                min_x: 285.35294,
                min_y: 124.86487,
            }),
        ),
        (
            ItemId(1953),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 124.86487,
                min_x: 279.52942,
                min_y: 124.17392,
            }),
        ),
        (
            ItemId(1954),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 18.626087,
                min_x: 244.38025,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1955),
            Item::Rect(Rect {
                max_x: 246.04411,
                max_y: 18.626087,
                min_x: 244.58823,
                min_y: 18.26087,
            }),
        ),
        (
            ItemId(1956),
            Item::Rect(Rect {
                max_x: 246.04411,
                max_y: 18.26087,
                min_x: 244.58823,
                min_y: 16.434782,
            }),
        ),
        (
            ItemId(1957),
            Item::Rect(Rect {
                max_x: 244.58823,
                max_y: 18.26087,
                min_x: 244.38025,
                min_y: 16.434782,
            }),
        ),
        (
            ItemId(1958),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 167.66798,
                min_x: 289.0588,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(1959),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 63.913044,
                min_x: 289.72058,
                min_y: 63.561874,
            }),
        ),
        (
            ItemId(1960),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 68.478264,
                min_x: 289.72058,
                min_y: 63.913044,
            }),
        ),
        (
            ItemId(1961),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 109.565216,
                min_x: 110.4592,
                min_y: 109.250374,
            }),
        ),
        (
            ItemId(1962),
            Item::Rect(Rect {
                max_x: 110.64706,
                max_y: 109.85975,
                min_x: 110.4592,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1963),
            Item::Rect(Rect {
                max_x: 110.84117,
                max_y: 109.85975,
                min_x: 110.64706,
                min_y: 109.565216,
            }),
        ),
        (
            ItemId(1964),
            Item::Rect(Rect {
                max_x: 110.84117,
                max_y: 109.565216,
                min_x: 110.64706,
                min_y: 109.250374,
            }),
        ),
        (
            ItemId(1965),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 164.34782,
                min_x: 289.0588,
                min_y: 161.02768,
            }),
        ),
        (
            ItemId(1966),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 176.43224,
                min_x: 4.0441175,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1967),
            Item::Rect(Rect {
                max_x: 7.7076125,
                max_y: 176.43224,
                min_x: 5.8235292,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1968),
            Item::Rect(Rect {
                max_x: 7.7076125,
                max_y: 173.47827,
                min_x: 5.8235292,
                min_y: 170.68842,
            }),
        ),
        (
            ItemId(1969),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 173.47827,
                min_x: 4.0441175,
                min_y: 170.68842,
            }),
        ),
        (
            ItemId(1970),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 139.56522,
                min_x: 266.26468,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1971),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 140.16452,
                min_x: 267.88235,
                min_y: 139.56522,
            }),
        ),
        (
            ItemId(1972),
            Item::Rect(Rect {
                max_x: 277.81662,
                max_y: 146.08696,
                min_x: 273.70587,
                min_y: 140.16452,
            }),
        ),
        (
            ItemId(1973),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 148.77237,
                min_x: 277.81662,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1974),
            Item::Rect(Rect {
                max_x: 281.29413,
                max_y: 148.77237,
                min_x: 279.52942,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1975),
            Item::Rect(Rect {
                max_x: 281.47058,
                max_y: 146.08696,
                min_x: 281.29413,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1976),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 136.95653,
                min_x: 281.47058,
                min_y: 130.51152,
            }),
        ),
        (
            ItemId(1977),
            Item::Rect(Rect {
                max_x: 286.92688,
                max_y: 130.51152,
                min_x: 285.35294,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1978),
            Item::Rect(Rect {
                max_x: 286.92688,
                max_y: 127.82609,
                min_x: 285.35294,
                min_y: 125.358406,
            }),
        ),
        (
            ItemId(1979),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 125.358406,
                min_x: 279.52942,
                min_y: 124.63043,
            }),
        ),
        (
            ItemId(1980),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 124.86487,
                min_x: 273.70587,
                min_y: 124.63043,
            }),
        ),
        (
            ItemId(1981),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 127.82609,
                min_x: 271.7647,
                min_y: 124.86487,
            }),
        ),
        (
            ItemId(1982),
            Item::Rect(Rect {
                max_x: 271.7647,
                max_y: 134.2711,
                min_x: 267.88235,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1983),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 136.95653,
                min_x: 266.26468,
                min_y: 134.2711,
            }),
        ),
        (
            ItemId(1984),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 164.34782,
                min_x: 289.41174,
                min_y: 161.58104,
            }),
        ),
        (
            ItemId(1985),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 167.11462,
                min_x: 289.41174,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(1986),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 175.89514,
                min_x: 4.367647,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1987),
            Item::Rect(Rect {
                max_x: 7.3650517,
                max_y: 175.89514,
                min_x: 5.8235292,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(1988),
            Item::Rect(Rect {
                max_x: 7.3650517,
                max_y: 173.47827,
                min_x: 5.8235292,
                min_y: 171.19566,
            }),
        ),
        (
            ItemId(1989),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 173.47827,
                min_x: 4.367647,
                min_y: 171.19566,
            }),
        ),
        (
            ItemId(1990),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 139.67097,
                min_x: 267.88235,
                min_y: 139.04349,
            }),
        ),
        (
            ItemId(1991),
            Item::Rect(Rect {
                max_x: 278.15915,
                max_y: 146.08696,
                min_x: 273.70587,
                min_y: 139.67097,
            }),
        ),
        (
            ItemId(1992),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 148.2353,
                min_x: 278.15915,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1993),
            Item::Rect(Rect {
                max_x: 280.94116,
                max_y: 148.2353,
                min_x: 279.52942,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(1994),
            Item::Rect(Rect {
                max_x: 281.14706,
                max_y: 146.08696,
                min_x: 280.94116,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(1995),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 136.95653,
                min_x: 281.14706,
                min_y: 129.97443,
            }),
        ),
        (
            ItemId(1996),
            Item::Rect(Rect {
                max_x: 286.6121,
                max_y: 129.97443,
                min_x: 285.35294,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(1997),
            Item::Rect(Rect {
                max_x: 286.6121,
                max_y: 127.82609,
                min_x: 285.35294,
                min_y: 125.851944,
            }),
        ),
        (
            ItemId(1998),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 125.851944,
                min_x: 279.52942,
                min_y: 125.08696,
            }),
        ),
        (
            ItemId(1999),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 125.358406,
                min_x: 273.70587,
                min_y: 125.08696,
            }),
        ),
        (
            ItemId(2000),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 127.82609,
                min_x: 272.08823,
                min_y: 125.358406,
            }),
        ),
        (
            ItemId(2001),
            Item::Rect(Rect {
                max_x: 272.08823,
                max_y: 134.80818,
                min_x: 267.88235,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(2002),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 136.95653,
                min_x: 266.58823,
                min_y: 134.80818,
            }),
        ),
        (
            ItemId(2003),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 139.04349,
                min_x: 266.58823,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(2004),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 166.56126,
                min_x: 289.7647,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(2005),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 164.34782,
                min_x: 289.7647,
                min_y: 162.1344,
            }),
        ),
        (
            ItemId(2006),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 175.35806,
                min_x: 4.6911764,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(2007),
            Item::Rect(Rect {
                max_x: 7.022491,
                max_y: 175.35806,
                min_x: 5.8235292,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(2008),
            Item::Rect(Rect {
                max_x: 7.022491,
                max_y: 173.47827,
                min_x: 5.8235292,
                min_y: 171.7029,
            }),
        ),
        (
            ItemId(2009),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 173.47827,
                min_x: 4.6911764,
                min_y: 171.7029,
            }),
        ),
        (
            ItemId(2010),
            Item::Rect(Rect {
                max_x: 286.2973,
                max_y: 129.43735,
                min_x: 285.35294,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(2011),
            Item::Rect(Rect {
                max_x: 286.2973,
                max_y: 127.82609,
                min_x: 285.35294,
                min_y: 126.34548,
            }),
        ),
        (
            ItemId(2012),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 126.34548,
                min_x: 279.52942,
                min_y: 125.54348,
            }),
        ),
        (
            ItemId(2013),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 125.851944,
                min_x: 273.70587,
                min_y: 125.54348,
            }),
        ),
        (
            ItemId(2014),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 127.82609,
                min_x: 272.41177,
                min_y: 125.851944,
            }),
        ),
        (
            ItemId(2015),
            Item::Rect(Rect {
                max_x: 272.41177,
                max_y: 135.34528,
                min_x: 267.88235,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(2016),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 136.95653,
                min_x: 266.91174,
                min_y: 135.34528,
            }),
        ),
        (
            ItemId(2017),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 138.52174,
                min_x: 266.91174,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(2018),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 139.17744,
                min_x: 267.88235,
                min_y: 138.52174,
            }),
        ),
        (
            ItemId(2019),
            Item::Rect(Rect {
                max_x: 278.5017,
                max_y: 146.08696,
                min_x: 273.70587,
                min_y: 139.17744,
            }),
        ),
        (
            ItemId(2020),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 147.69821,
                min_x: 278.5017,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(2021),
            Item::Rect(Rect {
                max_x: 280.58823,
                max_y: 147.69821,
                min_x: 279.52942,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(2022),
            Item::Rect(Rect {
                max_x: 280.82352,
                max_y: 146.08696,
                min_x: 280.58823,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(2023),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 136.95653,
                min_x: 280.82352,
                min_y: 129.43735,
            }),
        ),
        (
            ItemId(2024),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 164.34782,
                min_x: 290.1176,
                min_y: 162.68776,
            }),
        ),
        (
            ItemId(2025),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 166.0079,
                min_x: 290.1176,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(2026),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 174.82097,
                min_x: 5.0147057,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(2027),
            Item::Rect(Rect {
                max_x: 6.6799307,
                max_y: 174.82097,
                min_x: 5.8235292,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(2028),
            Item::Rect(Rect {
                max_x: 6.6799307,
                max_y: 173.47827,
                min_x: 5.8235292,
                min_y: 172.21014,
            }),
        ),
        (
            ItemId(2029),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 173.47827,
                min_x: 5.0147057,
                min_y: 172.21014,
            }),
        ),
        (
            ItemId(2030),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 127.82609,
                min_x: 272.7353,
                min_y: 126.34548,
            }),
        ),
        (
            ItemId(2031),
            Item::Rect(Rect {
                max_x: 272.7353,
                max_y: 135.88235,
                min_x: 267.88235,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(2032),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 136.95653,
                min_x: 267.2353,
                min_y: 135.88235,
            }),
        ),
        (
            ItemId(2033),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 138.0,
                min_x: 267.2353,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(2034),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 138.6839,
                min_x: 267.88235,
                min_y: 138.0,
            }),
        ),
        (
            ItemId(2035),
            Item::Rect(Rect {
                max_x: 278.8443,
                max_y: 146.08696,
                min_x: 273.70587,
                min_y: 138.6839,
            }),
        ),
        (
            ItemId(2036),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 147.16113,
                min_x: 278.8443,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(2037),
            Item::Rect(Rect {
                max_x: 280.2353,
                max_y: 147.16113,
                min_x: 279.52942,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(2038),
            Item::Rect(Rect {
                max_x: 280.5,
                max_y: 146.08696,
                min_x: 280.2353,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(2039),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 136.95653,
                min_x: 280.5,
                min_y: 128.90025,
            }),
        ),
        (
            ItemId(2040),
            Item::Rect(Rect {
                max_x: 285.9825,
                max_y: 128.90025,
                min_x: 285.35294,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(2041),
            Item::Rect(Rect {
                max_x: 285.9825,
                max_y: 127.82609,
                min_x: 285.35294,
                min_y: 126.83901,
            }),
        ),
        (
            ItemId(2042),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 126.83901,
                min_x: 279.52942,
                min_y: 126.00001,
            }),
        ),
        (
            ItemId(2043),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 126.34548,
                min_x: 273.70587,
                min_y: 126.00001,
            }),
        ),
        (
            ItemId(2044),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 165.45454,
                min_x: 290.47058,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(2045),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 164.34782,
                min_x: 290.47058,
                min_y: 163.2411,
            }),
        ),
        (
            ItemId(2046),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 174.28389,
                min_x: 5.3382354,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(2047),
            Item::Rect(Rect {
                max_x: 6.33737,
                max_y: 174.28389,
                min_x: 5.8235292,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(2048),
            Item::Rect(Rect {
                max_x: 6.33737,
                max_y: 173.47827,
                min_x: 5.8235292,
                min_y: 172.71739,
            }),
        ),
        (
            ItemId(2049),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 173.47827,
                min_x: 5.3382354,
                min_y: 172.71739,
            }),
        ),
        (
            ItemId(2050),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 137.47827,
                min_x: 267.5588,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(2051),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 138.19037,
                min_x: 267.88235,
                min_y: 137.47827,
            }),
        ),
        (
            ItemId(2052),
            Item::Rect(Rect {
                max_x: 279.18686,
                max_y: 146.08696,
                min_x: 273.70587,
                min_y: 138.19037,
            }),
        ),
        (
            ItemId(2053),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 146.62404,
                min_x: 279.18686,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(2054),
            Item::Rect(Rect {
                max_x: 279.88235,
                max_y: 146.62404,
                min_x: 279.52942,
                min_y: 146.08696,
            }),
        ),
        (
            ItemId(2055),
            Item::Rect(Rect {
                max_x: 280.17645,
                max_y: 146.08696,
                min_x: 279.88235,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(2056),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 136.95653,
                min_x: 280.17645,
                min_y: 128.36317,
            }),
        ),
        (
            ItemId(2057),
            Item::Rect(Rect {
                max_x: 285.66772,
                max_y: 128.36317,
                min_x: 285.35294,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(2058),
            Item::Rect(Rect {
                max_x: 285.66772,
                max_y: 127.82609,
                min_x: 285.35294,
                min_y: 127.33255,
            }),
        ),
        (
            ItemId(2059),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 127.33255,
                min_x: 279.52942,
                min_y: 126.45653,
            }),
        ),
        (
            ItemId(2060),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 126.83901,
                min_x: 273.70587,
                min_y: 126.45653,
            }),
        ),
        (
            ItemId(2061),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 127.82609,
                min_x: 273.0588,
                min_y: 126.83901,
            }),
        ),
        (
            ItemId(2062),
            Item::Rect(Rect {
                max_x: 273.0588,
                max_y: 136.41943,
                min_x: 267.88235,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(2063),
            Item::Rect(Rect {
                max_x: 267.88235,
                max_y: 136.95653,
                min_x: 267.5588,
                min_y: 136.41943,
            }),
        ),
        (
            ItemId(2064),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 164.34782,
                min_x: 290.82352,
                min_y: 163.79446,
            }),
        ),
        (
            ItemId(2065),
            Item::Rect(Rect {
                max_x: 291.17645,
                max_y: 164.90118,
                min_x: 290.82352,
                min_y: 164.34782,
            }),
        ),
        (
            ItemId(2066),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 173.7468,
                min_x: 5.6617646,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(2067),
            Item::Rect(Rect {
                max_x: 5.9948096,
                max_y: 173.7468,
                min_x: 5.8235292,
                min_y: 173.47827,
            }),
        ),
        (
            ItemId(2068),
            Item::Rect(Rect {
                max_x: 5.9948096,
                max_y: 173.47827,
                min_x: 5.8235292,
                min_y: 173.22464,
            }),
        ),
        (
            ItemId(2069),
            Item::Rect(Rect {
                max_x: 5.8235292,
                max_y: 173.47827,
                min_x: 5.6617646,
                min_y: 173.22464,
            }),
        ),
        (
            ItemId(2070),
            Item::Rect(Rect {
                max_x: 279.85294,
                max_y: 146.08696,
                min_x: 279.52942,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(2071),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 136.95653,
                min_x: 279.85294,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(2072),
            Item::Rect(Rect {
                max_x: 285.35294,
                max_y: 127.82609,
                min_x: 279.52942,
                min_y: 126.91304,
            }),
        ),
        (
            ItemId(2073),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 127.33255,
                min_x: 273.70587,
                min_y: 126.91304,
            }),
        ),
        (
            ItemId(2074),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 127.82609,
                min_x: 273.38232,
                min_y: 127.33255,
            }),
        ),
        (
            ItemId(2075),
            Item::Rect(Rect {
                max_x: 273.38232,
                max_y: 136.95653,
                min_x: 267.88235,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(2076),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 137.69684,
                min_x: 267.88235,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(2077),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 146.08696,
                min_x: 273.70587,
                min_y: 137.69684,
            }),
        ),
        (
            ItemId(2078),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 137.2033,
                min_x: 273.70587,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(2079),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 136.95653,
                min_x: 279.52942,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(2080),
            Item::Rect(Rect {
                max_x: 282.44116,
                max_y: 136.95653,
                min_x: 279.52942,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(2081),
            Item::Rect(Rect {
                max_x: 282.44116,
                max_y: 127.82609,
                min_x: 279.52942,
                min_y: 127.36957,
            }),
        ),
        (
            ItemId(2082),
            Item::Rect(Rect {
                max_x: 279.52942,
                max_y: 127.82609,
                min_x: 273.70587,
                min_y: 127.36957,
            }),
        ),
        (
            ItemId(2083),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 136.95653,
                min_x: 271.7647,
                min_y: 127.82609,
            }),
        ),
        (
            ItemId(2084),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 137.2033,
                min_x: 271.7647,
                min_y: 136.95653,
            }),
        ),
        (
            ItemId(2085),
            Item::Rect(Rect {
                max_x: 273.70587,
                max_y: 127.82609,
                min_x: 273.70587,
                min_y: 127.82609,
            }),
        ),
    ];

    let now = Instant::now();
    println!("constructing quadtree...");
    let mut quadtree = QuadTree::new(); // QuadTree::new() now takes no arguments
    for (item_id, item) in items {
        // items vector is defined above
        quadtree.insert(item_id, item); // This will currently not insert if bbox is None
    }
    println!("constructed quadtree in {:?}", now.elapsed());
    let now = Instant::now();
    let result = quadtree.get_ids_that_overlap(&Rect {
        max_x: 195.15001 + 1.0,
        max_y: 137.0 + 1.0,
        min_x: 194.15001 - 1.0,
        min_y: 136.0 - 1.0,
    });
    println!("query took in {:?}", now.elapsed());
    println!("result: {:?}", result);
}
