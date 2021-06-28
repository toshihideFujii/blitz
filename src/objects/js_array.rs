// The JSArray describes JavaScript Arrays
// Such an array can be of two modes:
//   - fast, backing storage is a FixedArray and length <= elements.length();
//     Please note: push and pop can be used to grow and shrink the array.
//   - slow, backing storage is a HashTable with numbers as keys.
struct JSArray {}
