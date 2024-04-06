/// An array associating a cell index to the indices of its 1-range neighbours.
/// 
/// The array is composed of 45 segments of length 7.
/// 
/// The first element of each segment is the number of neighbours of the corresponding cell.
/// 
/// The rest of the elements are the indices of the neighbouring cells. Unused values are set to SIZE_MAX.
pub const NEIGHBOURS1: [usize; 315] = [
    3, 1, 6, 7, usize::MAX, usize::MAX, usize::MAX,
    4, 0, 2, 7, 8, usize::MAX, usize::MAX,
    4, 1, 3, 8, 9, usize::MAX, usize::MAX,
    4, 2, 4, 9, 10, usize::MAX, usize::MAX,
    4, 3, 5, 10, 11, usize::MAX, usize::MAX,
    3, 4, 11, 12, usize::MAX, usize::MAX, usize::MAX,
    3, 0, 7, 13, usize::MAX, usize::MAX, usize::MAX,
    6, 0, 1, 6, 8, 13, 14,
    6, 1, 2, 7, 9, 14, 15,
    6, 2, 3, 8, 10, 15, 16,
    6, 3, 4, 9, 11, 16, 17,
    6, 4, 5, 10, 12, 17, 18,
    3, 5, 11, 18, usize::MAX, usize::MAX, usize::MAX,
    5, 6, 7, 14, 19, 20, usize::MAX,
    6, 7, 8, 13, 15, 20, 21,
    6, 8, 9, 14, 16, 21, 22,
    6, 9, 10, 15, 17, 22, 23,
    6, 10, 11, 16, 18, 23, 24,
    5, 11, 12, 17, 24, 25, usize::MAX,
    3, 13, 20, 26, usize::MAX, usize::MAX, usize::MAX,
    6, 13, 14, 19, 21, 26, 27,
    6, 14, 15, 20, 22, 27, 28,
    6, 15, 16, 21, 23, 28, 29,
    6, 16, 17, 22, 24, 29, 30,
    6, 17, 18, 23, 25, 30, 31,
    3, 18, 24, 31, usize::MAX, usize::MAX, usize::MAX,
    5, 19, 20, 27, 32, 33, usize::MAX,
    6, 20, 21, 26, 28, 33, 34,
    6, 21, 22, 27, 29, 34, 35,
    6, 22, 23, 28, 30, 35, 36,
    6, 23, 24, 29, 31, 36, 37,
    5, 24, 25, 30, 37, 38, usize::MAX,
    3, 26, 33, 39, usize::MAX, usize::MAX, usize::MAX,
    6, 26, 27, 32, 34, 39, 40,
    6, 27, 28, 33, 35, 40, 41,
    6, 28, 29, 34, 36, 41, 42,
    6, 29, 30, 35, 37, 42, 43,
    6, 30, 31, 36, 38, 43, 44,
    3, 31, 37, 44, usize::MAX, usize::MAX, usize::MAX,
    3, 32, 33, 40, usize::MAX, usize::MAX, usize::MAX,
    4, 33, 34, 39, 41, usize::MAX, usize::MAX,
    4, 34, 35, 40, 42, usize::MAX, usize::MAX,
    4, 35, 36, 41, 43, usize::MAX, usize::MAX,
    4, 36, 37, 42, 44, usize::MAX, usize::MAX,
    3, 37, 38, 43, usize::MAX, usize::MAX, usize::MAX
];

/// An array associating a cell index to the indices of its 2-range neighbours.
/// 
/// The array is composed of 45 segments of length 7.
/// 
/// The first element of each segment is the number of neighbours of the corresponding cell.
/// 
/// The rest of the elements are the indices of the neighbouring cells. Unused values are set to SIZE_MAX.
pub const NEIGHBOURS2: [usize; 315] = [
    2, 2, 14, usize::MAX, usize::MAX, usize::MAX, usize::MAX,
    3, 3, 13, 15, usize::MAX, usize::MAX, usize::MAX,
    4, 0, 4, 14, 16, usize::MAX, usize::MAX,
    4, 1, 5, 15, 17, usize::MAX, usize::MAX,
    3, 2, 16, 18, usize::MAX, usize::MAX, usize::MAX,
    2, 3, 17, usize::MAX, usize::MAX, usize::MAX, usize::MAX,
    2, 8, 20, usize::MAX, usize::MAX, usize::MAX, usize::MAX,
    3, 9, 19, 21, usize::MAX, usize::MAX, usize::MAX,
    4, 6, 10, 20, 22, usize::MAX, usize::MAX,
    4, 7, 11, 21, 23, usize::MAX, usize::MAX,
    4, 8, 12, 22, 24, usize::MAX, usize::MAX,
    3, 9, 23, 25, usize::MAX, usize::MAX, usize::MAX,
    2, 10, 24, usize::MAX, usize::MAX, usize::MAX, usize::MAX,
    3, 1, 15, 27, usize::MAX, usize::MAX, usize::MAX,
    5, 0, 2, 16, 26, 28, usize::MAX,
    6, 1, 3, 13, 17, 27, 29,
    6, 2, 4, 14, 18, 28, 30,
    5, 3, 5, 15, 29, 31, usize::MAX,
    3, 4, 16, 30, usize::MAX, usize::MAX, usize::MAX,
    3, 7, 21, 33, usize::MAX, usize::MAX, usize::MAX,
    5, 6, 8, 22, 32, 34, usize::MAX,
    6, 7, 9, 19, 23, 33, 35,
    6, 8, 10, 20, 24, 34, 36,
    6, 9, 11, 21, 25, 35, 37,
    5, 10, 12, 22, 36, 38, usize::MAX,
    3, 11, 23, 37, usize::MAX, usize::MAX, usize::MAX,
    3, 14, 28, 40, usize::MAX, usize::MAX, usize::MAX,
    5, 13, 15, 29, 39, 41, usize::MAX,
    6, 14, 16, 26, 30, 40, 42,
    6, 15, 17, 27, 31, 41, 43,
    5, 16, 18, 28, 42, 44, usize::MAX,
    3, 17, 29, 43, usize::MAX, usize::MAX, usize::MAX,
    2, 20, 34, usize::MAX, usize::MAX, usize::MAX, usize::MAX,
    3, 19, 21, 35, usize::MAX, usize::MAX, usize::MAX,
    4, 20, 22, 32, 36, usize::MAX, usize::MAX,
    4, 21, 23, 33, 37, usize::MAX, usize::MAX,
    4, 22, 24, 34, 38, usize::MAX, usize::MAX,
    3, 23, 25, 35, usize::MAX, usize::MAX, usize::MAX,
    2, 24, 36, usize::MAX, usize::MAX, usize::MAX, usize::MAX,
    2, 27, 41, usize::MAX, usize::MAX, usize::MAX, usize::MAX,
    3, 26, 28, 42, usize::MAX, usize::MAX, usize::MAX,
    4, 27, 29, 39, 43, usize::MAX, usize::MAX,
    4, 28, 30, 40, 44, usize::MAX, usize::MAX,
    3, 29, 31, 41, usize::MAX, usize::MAX, usize::MAX,
    2, 30, 42, usize::MAX, usize::MAX, usize::MAX, usize::MAX
];