use crate::ChunkFrame;

fn update_sand_dl(frames: &mut [ChunkFrame],
                  this_frame_index: usize,
                  next_frame_index: usize,
                  x: usize,
                  y: usize) -> bool {
    if x > 0 {
        let dl_x = x-1;
        let dl_y = y+1;

        if frames[this_frame_index].cells[(dl_x, dl_y)] == 0 {
            // move down
            frames[next_frame_index].cells[(dl_x, dl_y)] = 3;
            return true;
        }
    }
    return false;
}


fn update_sand_dr(frames: &mut [ChunkFrame],
                  this_frame_index: usize,
                  next_frame_index: usize,
                  x: usize,
                  y: usize) -> bool {
    if x < 79 {
        let dr_x = x+1;
        let dr_y = y+1;

        if frames[this_frame_index].cells[(dr_x, dr_y)] == 0 {
            // move down
            frames[next_frame_index].cells[(dr_x, dr_y)] = 3;
            return true;
        }
    }
    return false;
}



pub fn update_sand(frames: &mut [ChunkFrame],
               this_frame_index: usize,
               next_frame_index: usize,
               x: usize,
               y: usize) {

    frames[next_frame_index].cells[(x,y)] = 0;

    // check down
    
    let down_x = x;
    let down_y = y+1;

    if frames[this_frame_index].cells[(down_x, down_y)] == 0 {
        // move down
        frames[next_frame_index].cells[(down_x, down_y)] = 3;
        return;
    }

    if macroquad::rand::rand() % 2 == 0 {
        // down-left
        if update_sand_dl(frames,
                          this_frame_index, next_frame_index,
                          x, y) {
            return;
        }
        
        // down-right
        if update_sand_dr(frames,
                          this_frame_index, next_frame_index,
                          x, y) {
            return;
        }                
    } else {
        
        // down-right
        if update_sand_dr(frames,
                          this_frame_index, next_frame_index,
                          x, y) {
            return;
        }                
        
        // down-left
        if update_sand_dl(frames,
                          this_frame_index, next_frame_index,
                          x, y) {
            return;
        }
    }

    // else, do not move
    frames[next_frame_index].cells[(x, y)] = 3;
}
