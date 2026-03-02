// DECLARATION OF 3D ARRAY OF STRING IN RUST
fn main() {
    let array_3d = [
        [
            ["Apple".to_string(),"Banana".to_string()],
            ["Guava".to_string(),"Pineapple".to_string()],
        ],
        [
            ["Elephant".to_string(),"Tiger".to_string()],
            ["Lion".to_string(),"Cheetah".to_string()],
        ],
        [
            ["Ludo".to_string(),"Cricket".to_string()],
            ["Badminton".to_string(),"Carrom".to_string()],
        ],
    ];
    println!("Element at [0][1][0]:{}",array_3d[0][1][0]);
    println!("Element at [2][0][1]:{}",array_3d[2][0][1]);
    for(i,array_2d) in array_3d.iter().enumerate() {
         for(j,row) in array_2d.iter().enumerate() {
            for(k,element) in row.iter().enumerate() {
                println!("Element at[{}][{}][{}]:{}",i,j,k,element);
            }
         }
    }
}
