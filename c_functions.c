#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <time.h>
#include <math.h>
#include <string.h>

/*
#Create library callable from Python
cc -fPIC -shared -O3 -o c_functions.so c_functions.c
*/

typedef signed char i8;
typedef unsigned char u8;

typedef short i16;
typedef unsigned short u16;

// typedef int i32;
typedef unsigned int u32;

typedef long long i64;
typedef unsigned long long u64;

void draw_sprites(
    u8 *,
    const int *,
    const int *,
    const int *,
    const int *,
    const int *,
    const int,
    const u8
);

void fill_maze(
    const int num_rows,
    const int num_cols,
    int map_filled[num_rows][num_cols],
    int un_walked_steps[num_rows][num_cols],
    const u8 map[num_rows][num_cols],
    const u8 walked[num_rows][num_cols],
    const int wide_start,
    const int wide_start_size,
    const int limit,
    const int start_row,
    const int start_col
)
{
    int size = num_rows * num_cols;

    // initialize map_filled
    for (int row = 0; row < num_rows; row++)
    {
        for (int col = 0; col < num_cols; col++)
        {
            map_filled[row][col] = -(map[row][col] == 0);
        }
    }

    // Initialize un_walked_steps as 0
    for (int row = 0; row < num_rows; row++)
    {
        for (int col = 0; col < num_cols; col++)
        {
            un_walked_steps[row][col] = 0;
        }
    }

    // Fill edges
    for (int col = 0; col < num_cols; col++)
    {
        map_filled[0][col] = 255; // Fill top
        map_filled[num_rows -1][col] = 255; // Fill bottom
    }
    
    for (int row = 0; row < num_rows; row++)
    {
        map_filled[row][0] = 255; // Fill left
        map_filled[row][num_cols - 1] = 255; // Fill right
    }

    int max_positions = (num_rows * 2) + (num_cols * 2); // The highest number of positions one step can have is if it fills all the outer edges.
    if (wide_start == 1)
    {
        max_positions += (wide_start_size * 2) * (wide_start_size * 2);
    }

    int *current_positions_rows = malloc(max_positions * sizeof(int));
    int *current_positions_cols = malloc(max_positions * sizeof(int));
    int *next_positions_rows = malloc(max_positions * sizeof(int));
    int *next_positions_cols = malloc(max_positions * sizeof(int));

    map_filled[start_row][start_col] = 1;

    current_positions_rows[0] = start_row;
    current_positions_cols[0] = start_col;

    int current_positions_length = 1;
    int next_positions_length;

    if (wide_start == 1)
    {
        for (int row = start_row - wide_start_size; row < start_row + wide_start_size; row++)
        {
            for (int col = start_col - wide_start_size; col < start_col + wide_start_size; col++)
            {
                map_filled[row][col] = 1;
                current_positions_rows[current_positions_length] = row;
                current_positions_cols[current_positions_length] = col;
                current_positions_length++;
            }
        }
    }

    int directions_rows[4] = {-1, 0, 1, 0};
    int directions_cols[4] = {0, 1, 0, -1};

    int *temp;

    for (int step_num = 2; step_num < limit; step_num++)
    {
        if (current_positions_length > 0) // If we have any steps to take
        {
            next_positions_length = 0;
            for (int i = 0; i < current_positions_length; i++)
            {
                for (int direction = 0; direction < 4; direction++)
                {
                    int next_position_row = current_positions_rows[i] + directions_rows[direction];
                    int next_position_col = current_positions_cols[i] + directions_cols[direction];

                    if (map_filled[next_position_row][next_position_col] == -1) // -1 means that the cell is NOT a wall or other unwalkable element
                    {
                        map_filled[next_position_row][next_position_col] = step_num;
                        next_positions_rows[next_positions_length] = next_position_row;
                        next_positions_cols[next_positions_length] = next_position_col;
                        next_positions_length += 1;
                    }

                    un_walked_steps[next_position_row][next_position_col] = un_walked_steps[current_positions_rows[i]][current_positions_cols[i]] + (walked[next_position_row][next_position_col] != 140); // TODO Fix
                }
            }
        }
        else
        {
            printf("Breaking at step_num %d\n", step_num);
            break;
        }

        // Set next positions as current position so they are ready for next loop
        temp = current_positions_rows;
        current_positions_rows = next_positions_rows;
        next_positions_rows = temp;

        temp = current_positions_cols;
        current_positions_cols = next_positions_cols;
        next_positions_cols = temp;

        current_positions_length = next_positions_length;
    }
    free(current_positions_rows);
    free(current_positions_cols);
    free(next_positions_rows);
    free(next_positions_cols);
}

void get_sprite_offsets(
    int *best_match_index,
    const int *img,
    const int *window_offsets,
    const int *middle_indices,
    const int *sprite_values,
    const int *sprite_indices,
    const int *sprite_indices_lengths,
    const int num_sprites
)
{
    int found_sprite;
    int match_counter;
    int sprite_index_offset;
    int window_offset;
    int best_match_count = 0;

    sprite_index_offset = 0;

    for (int o = 0; o < 32; o++) // Loop through row, col offset combinations
    {
        match_counter = 0;
        for (int i = 0; i < 1225; i++)
        {
            window_offset = window_offsets[(o * 13916) + middle_indices[i]];
            sprite_index_offset = 0;
            for (int k = 0; k < num_sprites; k++) // Loop through sprites
            {
                found_sprite = 1;
                for (int j = 0; j < 3; j++) // Match sprite with window
                {
                    if (img[window_offset + sprite_indices[sprite_index_offset + j]] != sprite_values[sprite_index_offset + j])
                    {
                        found_sprite = 0;
                        break;
                    }
                }

                if (found_sprite == 1)
                {
                    match_counter++;
                }

                sprite_index_offset += sprite_indices_lengths[k];
            }
        }

        if (match_counter > best_match_count)
        {
            best_match_count = match_counter;
            best_match_index[0] = o;
        }
    }
}

void find_matching_sprites(
    int *sprite_counts,
    int *window_indices,
    const int *img,
    const int *window_offsets,
    const int *sprite_values,
    const int *sprite_indices,
    const int *sprite_indices_lengths,
    const int *modulus,
    const int *modulo_indices,
    const int num_sprites,
    const int max_matches
)
{
    int sprite_index_offset = 0;

    int pixel_match_count;
    int overlap_match_count;

    int sprite_found_count;
    int sprites_found_count = 0;

    for (int k = 0; k < num_sprites; k++) // Loop through sprites
    {
        sprite_found_count = 0;
        for (int i = 0; i < 13916; i++) // Loop through window_offsets
        {
            pixel_match_count = 0;
            overlap_match_count = 0;
            for (int j = 0; j < sprite_indices_lengths[k]; j++) // Match sprite with window
            {
                int value = img[window_offsets[i] + sprite_indices[sprite_index_offset + j]];

                if (value == sprite_values[sprite_index_offset + j])
                {
                    pixel_match_count += 1;
                }
                else if (modulus[(value * 32) + modulo_indices[sprite_index_offset + j]] == 1)
                {
                    overlap_match_count += 1;
                }
                else
                {
                    break;
                }
            }

            if ((pixel_match_count + overlap_match_count) == sprite_indices_lengths[k] && (((double)pixel_match_count / (double)sprite_indices_lengths[k]) > 0.4))
            {
                window_indices[sprites_found_count] = i;
                sprites_found_count++;
                sprite_found_count++;

                if (sprites_found_count >= max_matches)
                {
                    sprite_counts[k] = sprite_found_count;
                    return;
                }
            }
        }
        sprite_counts[k] = sprite_found_count;
        sprite_index_offset += sprite_indices_lengths[k];
    }
}

void draw_sprites(
    u8 *img,
    const int *sprite_counts,
    const int *window_indices,
    const int *window_offsets,
    const int *sprite_indices,
    const int *sprite_indices_lengths,
    const int num_sprites,
    const u8 draw_color
)
{
    int window_index_offset = 0;
    int sprite_index_offset = 0;

    for (int k = 0; k < num_sprites; k++) // Loop through sprites
    {
        for (int j = window_index_offset; j < window_index_offset + sprite_counts[k]; j++) // Loop through each of the windows where we have found k sprite
        {
            for (int i = sprite_index_offset; i < sprite_index_offset + sprite_indices_lengths[k]; i++) // Loop through each of the walkable pixels in the sprite
            {
                img[window_offsets[window_indices[j]] + sprite_indices[i]] = draw_color;
            }
        }

        window_index_offset += sprite_counts[k];
        sprite_index_offset += sprite_indices_lengths[k];
    }
}

void draw_matching_sprites(
    u8 *result,
    const int *sprite_counts,
    const int *window_indices,
    const int *window_offsets,
    const int *sprite_indices,
    const int *sprite_indices_lengths,
    const int *sprite_indices_walkable,
    const int *sprite_indices_lengths_walkable,
    const int num_sprites
)
{
    draw_sprites(result, sprite_counts, window_indices, window_offsets, sprite_indices, sprite_indices_lengths, num_sprites, 1);
    draw_sprites(result, sprite_counts, window_indices, window_offsets, sprite_indices_walkable, sprite_indices_lengths_walkable, num_sprites, 0);
}

void get_middle_indices(int *middle_indices)
{
    int counter = 0;
    int counter_ = 0;
    
    for (int row = 0; row < 600 - 32; row += 4)
    {
        for (int col = 0; col < 800 - 16; col += 8)
        {
            if (row > 200 && row < 400 && col > 300 && col < 500)
            {
                middle_indices[counter] = counter_;
                counter ++;
            }

            counter_++;
        }
    }
}

void get_window_offsets(int *window_offsets)
{
    unsigned int counter = 0;
    for (int row_offset = 0; row_offset < 4; row_offset++)
    {
        for (int col_offset = 0; col_offset < 8; col_offset++)
        {
            for (int row = row_offset; row < 600 - 32; row += 4)
            {
                for (int col = col_offset; col < 800 - 16; col += 8)
                {
                    window_offsets[counter] = (row * 800) + col;
                    counter++;
                }
            }
        }
    }
}

void get_window_offsets_(
    int *window_offsets,
    const int row_max,
    const int col_max,
    const int window_row_size,
    const int window_col_size
)
{
    int num_rows = row_max - window_row_size + 1;
    int num_cols = col_max - window_col_size + 1;
    int num_windows = num_rows * num_cols;

    for (int row = 0; row < num_rows; row++)
    {
        for (int col = 0; col < num_cols; col++)
        {
            window_offsets[(row * num_cols) + col] = (row * col_max) + col;
        }
    }
}

void find_symbols_core(
    int *sprite_counts,
    int *window_indices,
    const int *img,
    const int *window_offsets,
    const int *sprite_indices,
    const int *sprite_indices_lengths,
    const int *window_symbol_offset_indices,
    const int num_sprites,
    const int num_windows,
    const int max_matches
)
{
    int sprite_index_offset = 0;

    int sprite_found_count;
    int sprites_found_count = 0;

    int found_sprite;

    for (int k = 0; k < num_sprites; k++) // Loop through sprites
    {
        sprite_found_count = 0;
        for (int i = window_symbol_offset_indices[k]; i < num_windows; i++) // Loop through window_offsets
        {
            found_sprite = 1;
            for (int j = sprite_index_offset; j < sprite_index_offset + sprite_indices_lengths[k]; j++) // Match sprite with window
            {
                if (img[window_offsets[i] + sprite_indices[j]] == 0)
                {
                    found_sprite = 0;
                    break;
                }
            }

            if (found_sprite == 1)
            {   
                window_indices[sprites_found_count] = i;
                sprites_found_count++;
                sprite_found_count++;

                if (sprites_found_count >= max_matches)
                {
                    sprite_counts[k] = sprite_found_count;
                    return;
                }
            }
        }

        sprite_counts[k] = sprite_found_count;
        sprite_index_offset += sprite_indices_lengths[k];
    }
}

void find_symbols(
    int *sprite_counts_result,
    int *window_indices_result,
    const int *img,
    const int *window_offsets,
    const int *window_offsets_check,
    const int *sprite_counts,
    const int *sprite_values,
    const int *sprite_indices,
    const int *sprite_indices_lengths,
    const int num_sprites,
    const int num_windows,
    const int max_matches
)
{
    int sprite_index_offset = 0;

    int sprite_found_count;
    int sprites_found_count = 0;
    int window_index_offset = 0;

    int found_sprite;

    for (int k = 0; k < num_sprites; k++) // Loop through sprites
    {
        sprite_found_count = 0;
        for (int i = window_index_offset; i < window_index_offset + sprite_counts[k]; i++) // Loop through each of the windows where we have found the core of k sprite
        {
            found_sprite = 1;
            for (int j = sprite_index_offset; j < sprite_index_offset + sprite_indices_lengths[k]; j++) // Match sprite with window
            {
                if (img[window_offsets[window_offsets_check[i]] + sprite_indices[j]] != sprite_values[j])
                {
                    found_sprite = 0;
                    break;
                }
            }

            if (found_sprite == 1)
            {
                window_indices_result[sprites_found_count] = window_offsets_check[i];
                sprites_found_count++;
                sprite_found_count++;

                if (sprites_found_count >= max_matches)
                {
                    sprite_counts_result[k] = sprite_found_count;
                    return;
                }
            }
        }

        sprite_counts_result[k] = sprite_found_count;
        sprite_index_offset += sprite_indices_lengths[k];
        window_index_offset += sprite_counts[k];
    }
}

void transform_image(
    int *result,
    const int *img,
    const int *core_colors,
    const int img_size,
    const int core_colors_size
)
{
    // Create transformation array
    int transformation_array[256];
    for (int i = 0; i < 256; i++)
    {
        transformation_array[i] = 0;
    }

    for (int i = 0; i < core_colors_size; i++)
    {
        transformation_array[core_colors[i]] = 1;
    }

    for (int i = 0; i < img_size; i++)
    {
        result[i] = transformation_array[img[i]];
    }
}

void get_window_offsets_symbol_indices(
    int *result,
    const int *sorted_window_sums,
    const int *images_core_color_counts,
    const int num_windows,
    const int num_colors
)
{
    int max_index = num_windows - 1;
    for (int i = num_colors - 1; i >= 0; i--)
    {
        for (int j = max_index; j >= 0; j--)
        {
            if (sorted_window_sums[j] < images_core_color_counts[i])
            {
                result[i] = j;
                max_index = j;
                break;
            }
        }
    }
}

void argsort(
    int *sorted_arr,
    const int *arr,
    const int arr_size,
    const int max_value
)
{
    int *counts = malloc(max_value * sizeof(int));
    int *offsets = malloc(max_value * sizeof(int));
    int offset = 0;
    int sum;

    // Initialize array values as zero
    for (int i = 0; i < max_value; i++)
    {
        counts[i] = 0;
    }

    // Count the number of occurances of each value
    for (int i = 0; i < arr_size; i++)
    {
        counts[arr[i]]++;
    }

    // Get the offsets of the counts array
    for (int i = 0; i < max_value; i++)
    {
        offsets[i] = offset;
        offset += counts[i];
    }

    // Insert the values into the sorted array
    for (int i = 0; i < arr_size; i++)
    {
        sum = arr[i];
        sorted_arr[offsets[sum]] = i;
        offsets[sum]++;
    }

    free(counts);
    free(offsets);
}

void panorama(
    int *result,
    const int *rows,
    const int *cols,
    const int *rows_,
    const int *cols_,
    const int *sprite_counts,
    const int *sprite_counts_,
    const int num_sprites
)
{
    int num_rows = 142;
    int num_cols = 98;
    int size = num_rows * 2 * num_cols * 2;

    int *diff_counts = malloc(size * sizeof(int));

    // Initialize array as zero
    for (int i = 0; i < size; i++)
    {
        diff_counts[i] = 0;
    }

    int sprite_counts_offset = 0;
    int sprite_counts_offset_ = 0;
    int index;
    int index_;
    int diff_row;
    int diff_col;
    int arr_row;
    int arr_col;

    for (int sprite_id = 0; sprite_id < num_sprites; sprite_id++)
    {
        for (int i = 0; i < sprite_counts[sprite_id]; i++)
        {
            index = sprite_counts_offset + i;
            for (int j = 0; j < sprite_counts_[sprite_id]; j++)
            {
                index_ = sprite_counts_offset_ + j;

                diff_row = rows[index] - rows_[index_];
                diff_col = cols[index] - cols_[index_];

                arr_row = ((diff_row >= 0) * 142) + abs(diff_row);
                arr_col = ((diff_col >= 0) * 98) + abs(diff_col);

                diff_counts[arr_row * (num_cols * 2) + arr_col]++;
            }
        }

        sprite_counts_offset += sprite_counts[sprite_id];
        sprite_counts_offset_ += sprite_counts_[sprite_id];
    }

    int max_value = 0;
    int max_index = 0;

    for (int i = 0; i < size; i++)
    {
        if (diff_counts[i] > max_value)
        {
            max_value = diff_counts[i];
            max_index = i;
        }
    }

    int max_row;
    int max_col;

    max_row = (max_index / (num_cols * 2));
    max_col = max_index % (num_cols * 2);

    if (max_row >= num_rows)
    {
        max_row -= num_rows;
    }
    else
    {
        max_row = -max_row;
    }

    if (max_col >= num_cols)
    {
        max_col -= num_cols;
    }
    else
    {
        max_col = -max_col;
    }

    result[0] = max_row;
    result[1] = max_col;

    free(diff_counts);
}

void get_distances_to_edge(
    double *result,
    const u8 *img,
    const double *square_roots,
    const int num_rows,
    const int num_cols
)
{
    int size = num_rows * num_cols;

    int *distances_to_edge_top = malloc(size * sizeof(int));
    int *distances_to_edge_bottom = malloc(size * sizeof(int));
    int *distances_to_edge_left = malloc(size * sizeof(int));
    int *distances_to_edge_right = malloc(size * sizeof(int));

    int row_offset;
    int distance;

    for (int row = 0; row < num_rows; row++)
    {
        row_offset = row * num_cols;
        distance = 0;
        for (int col = row_offset; col < row_offset + num_cols; col++)
        {
            distance++;
            distance -= distance * img[col];
            distances_to_edge_left[col] = distance;
        }
    }

    for (int row = 0; row < num_rows; row++)
    {
        row_offset = row * num_cols;
        distance = 0;
        for (int col = row_offset + num_cols - 1; col >= row_offset; col--)
        {
            distance++;
            distance -= distance * img[col];
            distances_to_edge_right[col] = distance;
        }
    }

    for (int col = 0; col < num_cols; col++)
    {
        distance = 0;
        for (int row = col; row < size; row += num_cols)
        {
            distance++;
            distance -= distance * img[row];
            distances_to_edge_top[row] = distance;
        }
    }

    for (int col = 0; col < num_cols; col++)
    {
        distance = 0;
        for (int row = size - (num_cols - col); row >= col; row -= num_cols)
        {
            distance++;
            distance -= distance * img[row];
            distances_to_edge_bottom[row] = distance;
        }
    }

    for (int i = 0; i < size; i++)
    {
        distances_to_edge_top[i] = (distances_to_edge_top[i] < 30) ? distances_to_edge_top[i] : 30;
        distances_to_edge_bottom[i] = (distances_to_edge_bottom[i] < 30) ? distances_to_edge_bottom[i] : 30;
        distances_to_edge_left[i] = (distances_to_edge_left[i] < 30) ? distances_to_edge_left[i] : 30;
        distances_to_edge_right[i] = (distances_to_edge_right[i] < 30) ? distances_to_edge_right[i] : 30;
    }

    for (int i = 0; i < size; i++)
    {
        result[i] = (square_roots[distances_to_edge_top[i]] +
                     square_roots[distances_to_edge_bottom[i]] +
                     square_roots[distances_to_edge_left[i]] +
                     square_roots[distances_to_edge_right[i]]) *
                    3;
    }

    free(distances_to_edge_top);
    free(distances_to_edge_bottom);
    free(distances_to_edge_left);
    free(distances_to_edge_right);
}

void get_game_window_location(
    const int num_rows,
    const int num_cols,
    const int screen[num_rows][num_cols],
    const int *match_row,
    int *result
)
{
    result[0] = -1;
    result[1] = -1;

    int is_row_match = 0;

    for (int row = 600; row < num_rows; row++)
    {
        for (int col = 0; col < num_cols - 800; col++)
        {
            is_row_match = 1;
            for (int col_ = 0; col_ < 800; col_++)
            {
                if (screen[row][col + col_] != match_row[col_])
                {
                    is_row_match = 0;
                    break;
                }
            }

            if (is_row_match == 1)
            {
                result[0] = row;
                result[1] = col;
                return;
            }
        }
    }
}

void get_diff_count(
    int *result,
    const u8 *arr_1,
    const u8 *arr_2,
    const int size
)
{
    int diff_count = 0;
    for (int i = 0; i < size; i++)
    {
        diff_count += (arr_1[i] != arr_2[i]);
    }
    
    result[0] = diff_count;
}

void get_window_sums(
    const int arr_row_size,
    const int arr_col_size,
    const int arr[arr_row_size][arr_col_size],
    const int window_row_size,
    const int window_col_size,
    int window_sums_temp[arr_row_size][arr_col_size],
    int window_sums[arr_row_size - (window_row_size - 1)][arr_col_size - (window_col_size - 1)]
)
{
    const int size = arr_row_size * arr_col_size;

    // Copy arr to window_sums
    memcpy(window_sums_temp, arr, size * sizeof(int));

    // Loop through each row
    for (int row = 0; row < arr_row_size; row++)
    {
        // Calculate cumulative sum for this row
        for (int col = 1; col < arr_col_size; col++)
        {
            window_sums_temp[row][col] += window_sums_temp[row][col - 1];
        }

        // Calculate window sums for this row
        for (int col = arr_col_size - 1; col >= window_col_size; col--)
        {
            window_sums_temp[row][col] -= window_sums_temp[row][col - window_col_size];
        }
    }

    // Calculate cumulative sums for each column
    for (int row = 1; row < arr_row_size; row++)
    {
        for (int col = window_col_size - 1; col < arr_col_size; col++)
        {
            window_sums_temp[row][col] += window_sums_temp[row - 1][col];
        }
    }

    // Calculate sums for each window
    for (int row = arr_row_size - 1; row >= window_row_size; row--)
    {
        for (int col = arr_col_size; col >= window_col_size - 1; col--)
        {
            window_sums[row - window_row_size + 1][col - window_col_size + 1] = window_sums_temp[row][col] - window_sums_temp[row - window_row_size][col];
        }
    }

    // Add top row
    for (int col = 0; col < arr_col_size - (window_col_size - 1); col++)
    {
        window_sums[0][col] = window_sums_temp[window_row_size - 1][col + window_col_size - 1];
    }

    // Add left column
    for (int row = 1; row < arr_row_size - (window_row_size - 1); row++)
    {
        window_sums[row][0] = window_sums_temp[row + window_row_size - 1][window_col_size - 1] - window_sums_temp[row - 1][window_col_size - 1];
    }
}

void find_waypoint_on_map(
    int *waypoint_locations_rows,
    int *waypoint_locations_cols,
    const int num_rows,
    const int num_columns,
    const int num_colors,
    const u8 arr[num_rows][num_columns][num_colors]
)
{
    int match_counter = 0;

    for (int row = 0; row < num_rows - 2; row++)
    {
        for (int col = 0; col < num_columns; col++)
        {
            if (
                arr[row + 0][col][0] == 252 &&
                arr[row + 0][col][1] == 184 &&
                arr[row + 0][col][2] == 144 &&

                arr[row + 1][col][0] == 252 &&
                arr[row + 1][col][1] == 204 &&
                arr[row + 1][col][2] == 168 &&

                arr[row + 2][col][0] == 216 &&
                arr[row + 2][col][1] == 96 &&
                arr[row + 2][col][2] == 36
                )
            {
                if (match_counter < 2)
                {
                    waypoint_locations_rows[match_counter] = row;
                    waypoint_locations_cols[match_counter] = col % num_columns;

                    match_counter++;
                }
            }
        }
    }
}

void transform_image_from_resurrected_to_classic(
    const int num_rows,
    const int num_columns,
    const int num_colors,
    const u8 large_array[num_rows][num_columns][num_colors],
    u8 small_array[num_rows / 2][num_columns / 2][num_colors]
)
{
    for (int row_large = 0, row_small = 0; row_large < num_rows; row_large += 2, row_small++)
    {
        for (int column_large = 0, column_small = 0; column_large < num_columns; column_large += 2, column_small++)
        {
            small_array[row_small][column_small][0] = large_array[row_large][column_large][0];
            small_array[row_small][column_small][1] = large_array[row_large][column_large][1];
            small_array[row_small][column_small][2] = large_array[row_large][column_large][2];
        }
    }
}

void empty_rectangle_in_image_array(
    const int num_rows,
    const int num_columns,
    const int num_colors,
    u8 arr[num_rows][num_columns][num_colors],
    const int left,
    const int right,
    const int top,
    const int bottom
)
{
    for (int row = top; row < bottom; row++)
    {
        for (int col = left; col < right; col++)
        {
            arr[row][col][0] = 0;
            arr[row][col][1] = 0;
            arr[row][col][2] = 0;
        }
    }
}

int main()
{
}