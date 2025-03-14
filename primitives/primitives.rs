

// Primitives in Rust:

    // Scalar Types:

        /*
            - Signed integers: i8, i16, i32, i64, i128, and isize (pointer size)
            - Unsigned integers: u8, u16, u32, u64, u128, and usize (pointer size)
            - Floating point: f32, f64
            - char Unicode scalar values like 'a', 'α', and '∞' (4 bytes each)
            - bool either true or false
            - The unit type (), whose only possible value is an empty tuble: ()


        

         */

    // Compound Types: 

        // Arrays like [1, 2, 3]
        // Tuples like (1, true)

        // Variables can be type annotated. Numbers may additionally be annotated
        // via a suffix or by default. Integers default to i32 and floats to f64. 
        // Note: Rust can also infer types from context.

        fn main() {
            // Variables can be type annotated.
            let logical: bool = true; 

            let a_float: f64 = 1.0; // Regular annotation
            let an_integer   = 5i32; // Suffix annotation

            // Or a default will be used.
            let default_float = 3.0; // 'f64'
            let default_int   = 7;   // 'i32'

            // A type can also be inferred from context.
            let mut inferred_type = 12; // Type i64 is inferred from another line
            inferred_type = 4294967296i64; 

            //A mutable variable's value can be changed.
            let mut mutable = 12; // Mutable 'i32' 
            mutable = 21; 

            // Error! The type of a variable can't be changed.
            // mutable = true;

            // Variables can be overwritten with shadowing. 
            let mutable = true; 

            /* Compound types - Array and tuple */

            // Array signature consists of Type T and length as [T; length].
            let my_array: [i32; 5] = [1, 2, 3, 4, 5];

            // Tuple is a collection of values of different types
            // and is constructed using parentheses ().
            let my_tuple = (5u32, 1u8, true, -5.04f32); 
        }