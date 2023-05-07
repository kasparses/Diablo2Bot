#include <stdlib.h>
#include <stdio.h>
#include <sys/stat.h>
#include <time.h>

/*
#Create library callable from Python
cc -fPIC -shared -O3 -o monster_detection.so monster_detection.c
*/

#define WINDOW_SIZE 16

typedef signed char i8;
typedef unsigned char u8;

typedef short i16;
typedef unsigned short u16;

// typedef int i32;
typedef unsigned int u32;

typedef long long i64;
typedef unsigned long long u64;

struct Global_Data gl;

typedef struct
{
    u8 *data;
    int size;
} _File;

typedef struct
{
    u16 row;
    u16 col;
    u16 sprite_id;
    u16 palette_id;
} Match;

typedef struct
{
    u16 row;
    u16 col;
} Point;

typedef struct
{
    Point point;
    u16 sprite_id;
    u8 values[WINDOW_SIZE];
} Data;

typedef struct
{
    int row_size;
    int col_size;
    int size;
    int row_offset;
    int col_offset;
    u8 *data;
} Sprite;

typedef struct
{
    int size;
    u32 *data;
} Hash_Map;

struct Global_Data
{
    Hash_Map hash_map;
    Data *data;

    Sprite *sprites;
    u8 *palettes;

    int window_row_size;
    int window_col_size;
};

typedef struct
{
    int num_monsters;
    int *num_palshifts_per_monster;
    int *num_composits_per_monster;
    int *num_sprites_per_monster;
    int **num_sprites_per_composit;

    u8 **monster_palshifts;
    int **monster_palshift_ids;
    u8 ***monster_composits;
} Sprite_Group;

typedef struct
{
    int start;
    int end;
} Slice;

typedef struct
{
    int open;
    int close;
} Counts;

_File read_file(char *path)
{
    _File file;

    struct stat path_stat;
    stat(path, &path_stat);
    file.size = path_stat.st_size;

    u8 *data = malloc(path_stat.st_size * sizeof(u8));
    file.data = data;

    FILE *fp = fopen(path, "rb");
    if (fp)
    {
        fread(data, path_stat.st_size, 1, fp);
        fclose(fp);
    }
    else
    {
        printf("path \"%s\" is not a valid path. Exiting program!\n", path);
        exit(1);
    }

    return file;
}

void create_transformation_array(
    const u8 arr[256][3],
    u8 transformation_arr[256][256][256])
{
    for (int i = 0; i < 256; i++)
    {
        transformation_arr[arr[i][0]][arr[i][1]][arr[i][2]] = i;
    }
}

void transform_image(
    const int num_rows,
    const int num_cols,
    const u8 arr[num_rows][num_cols][3],
    u8 arr_transformed[num_rows][num_cols],
    const u8 index_arr[256][256][256])
{
    for (int row = 0; row < num_rows; row++)
    {
        for (int col = 0; col < num_cols; col++)
        {
            arr_transformed[row][col] = index_arr
                [arr[row][col][0]]
                [arr[row][col][1]]
                [arr[row][col][2]];
        }
    }
}

u32 combine_bytes(u8 a, u8 b)
{
    u32 result = 0;

    result |= b;
    result <<= 8;
    result |= a;

    return result;
}

u32 get_num_sprites(
    const int num_monsters,
    int *num_composits,
    u8 ***composits)
{
    u32 num_sprites = 0;

    for (int i = 0; i < num_monsters; i++)
    {
        for (int j = 0; j < num_composits[i]; j++)
        {
            num_sprites += combine_bytes(composits[i][j][0], composits[i][j][1]);
        }
    }

    if (num_sprites >= 65536)
    {
        printf("%d is too many sprites. Exiting program!", num_sprites);
        exit(1);
    }

    return num_sprites;
}

u32 parse_composit_into_sprite_structs(
    u8 *composit,
    Sprite *sprites)
{
    int c = 0;

    u8 a = composit[c++];
    u8 b = composit[c++];
    u32 num_sprites = combine_bytes(a, b);

    for (int i = 0; i < num_sprites; i++)
    {
        sprites[i].row_size = composit[c++];
        sprites[i].col_size = composit[c++];

        sprites[i].size = (sprites[i].row_size * sprites[i].col_size);
    }

    for (int i = 0; i < num_sprites; i++)
    {
        sprites[i].row_offset = composit[c++];
        sprites[i].col_offset = composit[c++];
    }

    for (int i = 0; i < num_sprites; i++)
    {
        sprites[i].data = composit + c;
        c += sprites[i].size;
    }

    return num_sprites;
}

void parse_composits_into_sprite_structs(
    Sprite_Group *sg,
    Sprite *sprites)
{
    u32 sprites_offset = 0;

    for (int i = 0; i < sg->num_monsters; i++)
    {
        int num_sprites_per_monster = 0;
        for (int j = 0; j < sg->num_composits_per_monster[i]; j++)
        {
            int num_sprites_in_composit = parse_composit_into_sprite_structs(sg->monster_composits[i][j], sprites + sprites_offset);
            sprites_offset += num_sprites_in_composit;
            num_sprites_per_monster += num_sprites_in_composit;

            sg->num_sprites_per_composit[i][j] = num_sprites_in_composit;
        }
        sg->num_sprites_per_monster[i] = num_sprites_per_monster;
    }
}

void invert_array(
    int size,
    u8 *arr,
    u8 *arr_inverted)
{
    for (int i = 0; i < size; i++)
    {
        if (arr[i] == 0 || arr[i] == 172)
        {
            arr_inverted[i] = 1;
        }
        else
        {
            arr_inverted[i] = 0;
        }
    }
}

void get_window_sums(
    int arr_rows,
    int arr_cols,
    int window_rows,
    int window_cols,
    u8 *arr,
    u8 *window_sums)
{
    for (int row = 0; row < arr_rows - (window_rows - 1); row++)
    {
        for (int col = 0; col < arr_cols - (window_cols - 1); col++)
        {
            window_sums[(row * arr_cols) + col] = 0;
            for (int r = 0; r < window_rows; r++)
            {
                for (int c = 0; c < window_cols; c++)
                {
                    window_sums[(row * arr_cols) + col] += arr[((row + r) * arr_cols) + col + c];
                }
            }
        }
    }
}

void palette_transform_array(
    const int size,
    const u8 *arr,
    u8 *arr_shifted,
    const u8 *palette)
{
    for (int i = 0; i < size; i++)
    {
        arr_shifted[i] = palette[arr[i]];
    }
}

int get_points(
    int arr_rows,
    int arr_cols,
    int window_rows,
    int window_cols,
    u8 *arr,
    Point *points)
{
    int c = 0;
    for (int row = 0; row < arr_rows - (window_rows - 1); row++)
    {
        for (int col = 0; col < arr_cols - (window_cols - 1); col++)
        {
            if (arr[(row * arr_cols) + col] == 0)
            {
                points[c].row = row;
                points[c].col = col;
                c++;
            }
        }
    }
    return c;
}

void get_num_unique_values_in_windows(
    Point *points,
    int num_points,
    const int arr_row_size,
    const int arr_col_size,
    const u8 *arr,
    const int window_row_size,
    const int window_col_size,
    u8 *window_unique_counts,
    u8 *colors,
    u8 *colors_found)
{

    int window_count = 0;

    for (int i = 0; i < num_points; i++)
    {
        int row = points[i].row;
        int col = points[i].col;

        int num_uniques = 0;
        for (int window_row = 0; window_row < window_row_size; window_row++)
        {
            for (int window_col = 0; window_col < window_col_size; window_col++)
            {
                u8 color = arr[((row + window_row) * arr_col_size) + col + window_col];
                if (colors[color] == 0)
                {
                    colors[color] = 1;
                    colors_found[num_uniques] = color;
                    num_uniques++;
                }
            }
        }

        window_unique_counts[window_count++] = num_uniques;
        // Clear array for next window
        for (int i = 0; i < num_uniques; i++)
        {
            colors[colors_found[i]] = 0;
        }
    }
}

void argsort(
    u16 *sorted_arr,
    u16 *counts,
    u16 *offsets,
    const u8 *arr,
    const u16 arr_size,
    const u8 max_value)
{
    u16 offset = 0;
    u8 sum;

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
}

int is_point_overlapping_existing_point(
    const int num_rows,
    const int num_cols,
    u8 *arr,
    const Point point,
    const int num_window_rows,
    const int num_window_cols)
{
    for (int row = point.row; row < point.row + num_window_rows; row++)
    {
        for (int col = point.col; col < point.col + num_window_cols; col++)
        {
            if (arr[(row * num_cols) + col] == 1)
            {
                return 1;
            }
        }
    }

    return 0;
}

void draw_window(
    const int num_rows,
    const int num_cols,
    u8 *arr,
    const Point base_point,
    const int num_window_rows,
    const int num_window_cols,
    const u8 value)
{
    for (int row = base_point.row; row < base_point.row + num_window_rows; row++)
    {
        for (int col = base_point.col; col < base_point.col + num_window_cols; col++)
        {
            arr[(row * num_cols) + col] = value;
        }
    }
}

int min(int a, int b)
{
    if (a <= b)
    {
        return a;
    }
    return b;
}

int add_best_windows(
    Sprite s,
    const int num_unbroken_windows,
    const int num_points,
    u8 *sprite_windows,
    const Point *_points,
    const u16 *window_unique_counts_sorted_indices,
    const u8 *window_unique_counts,
    const u32 window_row_size,
    const u32 window_col_size,
    Point *points,
    const int count_offset)
{
    int count = 0;
    int reverse_sorted_indices_count = num_unbroken_windows - 1;

    while (count < num_points && reverse_sorted_indices_count >= 0)
    {
        int _is_point_overlapping_existing_point = is_point_overlapping_existing_point(s.row_size, s.col_size, sprite_windows, _points[window_unique_counts_sorted_indices[reverse_sorted_indices_count]], window_row_size, window_col_size);

        if (window_unique_counts[window_unique_counts_sorted_indices[reverse_sorted_indices_count]] < 5) // 7
        {
            break;
        }

        if (_is_point_overlapping_existing_point == 0)
        {
            points[count_offset + count] = _points[window_unique_counts_sorted_indices[reverse_sorted_indices_count]];
            draw_window(s.row_size, s.col_size, sprite_windows, points[count_offset + count], window_row_size, window_col_size, 1);

            count++;
        }

        reverse_sorted_indices_count--;
    }

    for (int c = 0; c < count; c++)
    {
        draw_window(s.row_size, s.col_size, sprite_windows, points[count_offset + c], window_row_size, window_col_size, 0);
    }

    return count;
}

int get_unbroken_windows(
    int max_number_of_windows,
    Sprite_Group sg,
    Point *points,
    int *num_windows_per_sprite,
    int *num_windows_per_monster,
    Sprite *sprites,
    u8 *palette,
    const u32 num_sprites,
    u32 max_windows_per_sprite,
    const u32 window_row_size,
    const u32 window_col_size)
{
    const u32 max_sprite_size = 256 * 256;

    u8 *array_palette_shifted = malloc(max_sprite_size * sizeof(u8));
    u8 *array_inverted = malloc(max_sprite_size * sizeof(u8));
    u8 *window_sums = malloc(max_sprite_size * sizeof(u8));
    u8 *window_unique_counts = malloc(max_sprite_size * sizeof(u8));
    u8 *colors = calloc(256, sizeof(u8));
    u8 *colors_found = malloc(256 * sizeof(u8));
    Point *_points = malloc(max_sprite_size * sizeof(Point));

    u16 *window_unique_counts_sorted_indices = malloc(max_sprite_size * sizeof(u16));
    u16 *window_unique_counts_sort_counts = malloc(max_sprite_size * sizeof(u16));
    u16 *window_unique_counts_sort_offsets = malloc(max_sprite_size * sizeof(u16));

    u8 *sprite_windows = calloc(max_sprite_size, sizeof(u8));

    int points_counter = 0;

    int sprite_offset = 0;

    const int max_num_windows_per_monster = (int)((double)max_number_of_windows / (double)sg.num_monsters);
    for (int i = 0; i < sg.num_monsters; i++)
    {
        const int max_num_windows_per_composit = (int)((double)max_num_windows_per_monster / (double)sg.num_composits_per_monster[i]);
        for (int j = 0; j < sg.num_composits_per_monster[i]; j++)
        {
            const int max_num_windows_per_sprite = min(max_windows_per_sprite, (int)((double)max_num_windows_per_composit / (double)sg.num_sprites_per_composit[i][j]));
            for (int k = 0; k < sg.num_sprites_per_composit[i][j]; k++)
            {
                Sprite s = sprites[sprite_offset];

                palette_transform_array(s.size, s.data, array_palette_shifted, palette);
                invert_array(s.size, array_palette_shifted, array_inverted);
                get_window_sums(s.row_size, s.col_size, window_row_size, window_col_size, array_inverted, window_sums);

                int num_unbroken_windows = get_points(s.row_size, s.col_size, window_row_size, window_col_size, window_sums, _points);

                get_num_unique_values_in_windows(_points, num_unbroken_windows, s.row_size, s.col_size, array_palette_shifted, window_row_size, window_col_size, window_unique_counts, colors, colors_found);
                argsort(window_unique_counts_sorted_indices, window_unique_counts_sort_counts, window_unique_counts_sort_offsets, window_unique_counts, num_unbroken_windows, (window_row_size * window_col_size));

                int num_points = min(num_unbroken_windows, max_num_windows_per_sprite);

                int count = add_best_windows(s, num_unbroken_windows, num_points, sprite_windows, _points, window_unique_counts_sorted_indices, window_unique_counts, window_row_size, window_col_size, points, points_counter);

                num_windows_per_sprite[sprite_offset] = count;
                num_windows_per_monster[i] += count;

                points_counter += count;
                sprite_offset++;
            }
        }
    }

    free(array_palette_shifted);
    free(array_inverted);
    free(window_sums);
    free(window_unique_counts);
    free(colors);
    free(colors_found);
    free(_points);
    free(window_unique_counts_sorted_indices);
    free(window_unique_counts_sort_counts);
    free(window_unique_counts_sort_offsets);
    free(sprite_windows);

    return points_counter;
}

void copy_range(
    u8 *values,
    const int num_rows,
    const int num_cols,
    const u8 *arr,
    Point point,
    const int window_row_size,
    const int window_col_size)
{
    int counter = 0;

    for (int row = point.row; row < point.row + window_row_size; row++)
    {
        for (int col = point.col; col < point.col + window_col_size; col++)
        {
            values[counter] = arr[(row * num_cols) + col];
            counter++;
        }
    }
}

void copy_sprite_windows_into_data_struct(
    const u32 num_sprites,
    Sprite *sprites,
    int *num_windows_per_sprite,
    const u32 window_row_size,
    const u32 window_col_size,
    Data *data,
    Point *points)
{
    int point_offset = 0;
    for (int i = 0; i < num_sprites; i++)
    {
        Sprite s = sprites[i];
        for (int j = point_offset; j < point_offset + num_windows_per_sprite[i]; j++)
        {
            data[j].point = points[j];
            data[j].sprite_id = i;

            copy_range(data[j].values, s.row_size, s.col_size, s.data, points[j], window_row_size, window_col_size);
        }
        point_offset += num_windows_per_sprite[i];
    }
}

Hash_Map create_hash_map(const int size)
{
    Hash_Map hash_map;
    const int extra_space = 20000;

    int *data = calloc(size + extra_space, sizeof(int));

    hash_map.data = data;
    hash_map.size = size;

    for (int i = 0; i < size + extra_space; i++)
    {
        data[i] = 0;
    }
    return hash_map;
}

int get_hash_key(
    const u8 values[WINDOW_SIZE])
{
    int value = (values[0] * 31412 +
                 values[1] * 123 +
                 values[2] * 440234 +
                 values[3] * 221 +
                 values[4] * 11653 +
                 values[5] * 87432 +
                 values[6] * 101 +
                 values[7] * 99234 +
                 values[8] * 2654 +
                 values[9] * 14123 +
                 values[10] * 105 +
                 values[11] * 6743 +
                 values[12] * 332 +
                 values[13] * 15654 +
                 values[14] * 12234 +
                 values[15] * 3154312);

    return value;
}

void insert_element(
    Hash_Map *hash_map,
    const u8 values[WINDOW_SIZE],
    const int palette_id,
    const int data_id)
{
    const int identical_limit = 30;

    int key = get_hash_key(values) % hash_map->size;

    int loop_count = 0;

    // Find next empty slot
    while ((hash_map->data[key] >> 11) != 0)
    {
        key++;
        loop_count++;

        if (loop_count > identical_limit)
        {
            return;
        }
    }

    u32 data = 0;
    data = (data_id << 11);
    data = (data | palette_id);

    // Insert item
    hash_map->data[key] = data;
}

int get_number_of_palettes(
    const int num_monsters,
    int *num_palshifts,
    int **palshift_ids,
    const int num_light_radius_palettes,
    const int num_unique_palettes)
{
    int num_palettes = num_light_radius_palettes + num_unique_palettes;

    for (int i = 0; i < num_monsters; i++)
    {
        for (int j = 0; j < num_palshifts[i]; j++)
        {
            int palshift_id = palshift_ids[i][j];

            if (palshift_id != 0)
            {
                num_palettes += num_light_radius_palettes;
            }
        }
    }

    return num_palettes;
}

int add_palettes(
    const u8 *palettes_src,
    u8 *palettes_dst,
    const int num_palettes,
    int offset)
{
    for (int i = 0; i < num_palettes; i++)
    {
        for (int j = 0; j < 256; j++)
        {
            palettes_dst[offset++] = palettes_src[(i * 256) + j];
        }
    }

    return offset;
}

int _add_palettes(
    const u8 *palettes_src,
    u8 *palettes_dst,
    const u8 *palette_transformation,
    const int num_palettes,
    int offset)
{
    for (int i = 0; i < num_palettes; i++)
    {
        for (int j = 0; j < 256; j++)
        {
            palettes_dst[offset++] = palettes_src[(i * 256) + palette_transformation[j]];
        }
    }

    return offset;
}

void *get_palettes(
    const int num_monsters,
    const int *num_palshifts,
    int **palshift_ids,
    u8 **palshifts,
    const u8 *act_palettes,
    const u8 *unique_palettes,
    u8 *palettes,
    const int num_light_radius_palettes,
    const int num_unique_palettes,
    const int light_radius_palette_start_id)
{
    int offset = 0;

    offset = add_palettes(act_palettes + (light_radius_palette_start_id * 256), palettes, num_light_radius_palettes, offset);
    offset = add_palettes(unique_palettes, palettes, num_unique_palettes, offset);

    for (int i = 0; i < num_monsters; i++)
    {
        for (int j = 0; j < num_palshifts[i]; j++)
        {
            int palshift_id = palshift_ids[i][j];

            if (palshift_id != 0)
            {
                offset = _add_palettes(act_palettes + (light_radius_palette_start_id * 256), palettes, palshifts[i] + (palshift_id * 256), num_light_radius_palettes, offset);
            }
        }
    }
}

void copy_sprite_windows_into_contiguous_array(
    const int num_windows,
    u8 *window_values,
    Data *data)
{
    int offset = 0;
    for (int i = 0; i < num_windows; i++)
    {
        for (int j = 0; j < WINDOW_SIZE; j++)
        {
            window_values[offset++] = data[i].values[j];
        }
    }
}

int get_num_windows(
    const int num_monsters,
    int *num_palshifts,
    const int num_light_radius_palettes,
    const int num_unique_palettes,
    int *num_windows_per_monster)
{
    int num_windows = 0;

    for (int i = 0; i < num_monsters; i++)
    {
        num_windows += num_windows_per_monster[i] * num_unique_palettes;
        num_windows += num_palshifts[i] * num_windows_per_monster[i] * num_light_radius_palettes;
    }

    return num_windows;
}

void add_windows_to_hash_map(
    int num_windows,
    Hash_Map hash_map,
    u8 *window_values_palette_shifted,
    int palette_id,
    int window_offset)
{
    for (int window_id = 0; window_id < num_windows; window_id++)
    {
        int is_window_black = 0;
        for (int m = 0; m < WINDOW_SIZE; m++)
        {
            if (
                window_values_palette_shifted[(window_id * WINDOW_SIZE) + m] == 0 ||
                window_values_palette_shifted[(window_id * WINDOW_SIZE) + m] == 172)
            {
                is_window_black = 1;
                break;
            }
        }

        if (is_window_black == 0)
        {
            insert_element(&hash_map, window_values_palette_shifted + (window_id * WINDOW_SIZE), palette_id, window_offset + window_id);
        }
    }
}

void set_data(
    Sprite_Group sg,
    u8 *act_palettes,
    u8 *unique_champion_palettes)
{
    const u32 num_sprites = get_num_sprites(sg.num_monsters, sg.num_composits_per_monster, sg.monster_composits);
    printf("num_sprites: %u\n", num_sprites);

    Sprite *sprites = malloc(num_sprites * sizeof(Sprite));
    parse_composits_into_sprite_structs(&sg, sprites);

    const int num_light_radius_palettes = 16;
    const int light_radius_palette_start_id = 36 - num_light_radius_palettes; // 35 is the last of the light_radius palettes
    const int num_unique_palettes = 30;
    const u32 window_row_size = 4;
    const u32 window_col_size = 4;
    const u32 max_windows_per_sprite = 20;

    const int max_number_of_windows = 2097152;

    Point *points = malloc(max_number_of_windows * sizeof(Point));
    int *num_windows_per_sprite = malloc(num_sprites * sizeof(int));
    int *num_windows_per_monster = calloc(sg.num_monsters, sizeof(int));

    int num_windows = get_unbroken_windows(max_number_of_windows, sg, points, num_windows_per_sprite, num_windows_per_monster, sprites, act_palettes + (light_radius_palette_start_id * 256), num_sprites, max_windows_per_sprite, window_row_size, window_col_size);

    Data *data = malloc(num_windows * sizeof(Data));
    copy_sprite_windows_into_data_struct(num_sprites, sprites, num_windows_per_sprite, window_row_size, window_col_size, data, points);

    int num_palettes = get_number_of_palettes(sg.num_monsters, sg.num_palshifts_per_monster, sg.monster_palshift_ids, num_light_radius_palettes, num_unique_palettes);
    u8 *palettes = malloc(num_palettes * 256 * sizeof(u8));
    printf("num_palettes: %d\n", num_palettes);
    get_palettes(sg.num_monsters, sg.num_palshifts_per_monster, sg.monster_palshift_ids, sg.monster_palshifts, act_palettes, unique_champion_palettes, palettes, num_light_radius_palettes, num_unique_palettes, light_radius_palette_start_id);

    int num_windows_total = get_num_windows(sg.num_monsters, sg.num_palshifts_per_monster, num_light_radius_palettes, num_unique_palettes, num_windows_per_monster);
    const int hash_map_size = num_windows_total * 3; // TODO is * 3 too much? Maybe a little less buffer space is enough.
    printf("hash_map_size: %d\n", hash_map_size);
    Hash_Map hash_map = create_hash_map(hash_map_size);

    u8 *window_values = malloc(num_windows * WINDOW_SIZE * sizeof(u8));
    u8 *window_values_palette_shifted = malloc(num_windows * WINDOW_SIZE * sizeof(u8));

    copy_sprite_windows_into_contiguous_array(num_windows, window_values, data);

    int windows_offset = 0;
    int palette_offset = num_light_radius_palettes + num_unique_palettes;

    for (int monster_id = 0; monster_id < sg.num_monsters; monster_id++)
    {
        int num_windows = num_windows_per_monster[monster_id];
        int num_pixels = num_windows * WINDOW_SIZE;
        u8 *window_values_ptr = window_values + windows_offset * WINDOW_SIZE;

        for (int palette_id = num_light_radius_palettes; palette_id < num_light_radius_palettes + num_unique_palettes; palette_id++)
        {
            u8 *palette_ptr = palettes + (palette_id * 256);

            palette_transform_array(num_pixels, window_values_ptr, window_values_palette_shifted, palette_ptr);
            add_windows_to_hash_map(num_windows, hash_map, window_values_palette_shifted, palette_id, windows_offset);
        }

        for (int j = 0; j < sg.num_palshifts_per_monster[monster_id]; j++)
        {
            int palshift_id = sg.monster_palshift_ids[monster_id][j];

            if (palshift_id == 0)
            {
                for (int palette_id = 0; palette_id < num_light_radius_palettes; palette_id++)
                {
                    u8 *palette_ptr = palettes + (palette_id * 256);

                    palette_transform_array(num_pixels, window_values_ptr, window_values_palette_shifted, palette_ptr);
                    add_windows_to_hash_map(num_windows, hash_map, window_values_palette_shifted, palette_id, windows_offset);
                }
            }
            else
            {
                for (int palette_id = palette_offset; palette_id < palette_offset + num_light_radius_palettes; palette_id++)
                {
                    u8 *palette_ptr = palettes + (palette_id * 256);

                    palette_transform_array(num_pixels, window_values_ptr, window_values_palette_shifted, palette_ptr);
                    add_windows_to_hash_map(num_windows, hash_map, window_values_palette_shifted, palette_id, windows_offset);
                }

                palette_offset += num_light_radius_palettes;
            }
        }
        windows_offset += num_windows;
    }

    gl.data = data;
    gl.hash_map = hash_map;
    gl.palettes = palettes;
    gl.sprites = sprites;
    gl.window_row_size = window_row_size;
    gl.window_col_size = window_col_size;

    free(num_windows_per_sprite);
    free(points);
    free(window_values);
    free(window_values_palette_shifted);
}

int are_values_different(
    const u8 a[WINDOW_SIZE],
    const u8 b[WINDOW_SIZE],
    const u8 palette[256])
{
    int return_value = 0;
    for (int i = 0; i < WINDOW_SIZE; i++)
    {
        if (
            palette[a[i]] !=
            b[i])
        {
            return_value = 1;
            break;
        }
    }
    return return_value;
}

int find_element(
    const Hash_Map hash_map,
    const Data *data,
    const u8 values[WINDOW_SIZE],
    const u8 *palettes,
    const int max_matches,
    int *matching_keys)
{
    int match_count = 0;

    int key = get_hash_key(values);

    if (key > 0)
    {
        key = key % hash_map.size;

        u32 data_id;
        u32 palette_id;

        do
        {
            data_id = (hash_map.data[key] >> 11);
            palette_id = (hash_map.data[key] & 0x7FF);

            if (are_values_different(data[data_id].values, values, palettes + (palette_id * 256)) == 0)
            {
                matching_keys[match_count++] = key;
            }
            key++;

        } while (data_id != 0 && match_count < max_matches);
    }

    return match_count;
}

int validate_sprite(
    const int sprite_row_size,
    const int sprite_col_size,

    const u8 *sprite,

    const int arr_row_size,
    const int arr_col_size,
    const u8 *arr,

    const u8 palette[256],

    const int offset_row,
    const int offset_col)
{
    if (offset_row < 0 || offset_col < 0)
    {
        return 0;
    }

    int total_count = 0;
    int match_count = 0;

    for (int row = 0; row < min(arr_row_size - offset_row, sprite_row_size); row++)
    {
        for (int col = 0; col < min(arr_col_size - offset_col, sprite_col_size); col++)
        {
            if (sprite[(row * sprite_col_size) + col] != 0)
            {
                // Check if sprite pixel matches image pixel
                if (palette[sprite[(row * sprite_col_size) + col]] == arr[((row + offset_row) * arr_col_size) + col + offset_col])
                {
                    match_count++;
                }
                total_count++;
            }
        }
    }

    if (total_count > 0)
    {
        if ((double)((double)match_count / (double)total_count) > 0.3)
        {
            return 1;
        }
    }

    return 0;
}

void draw_sprite(
    const int sprite_row_size,
    const int sprite_col_size,

    const u8 *sprite,

    const int arr_row_size,
    const int arr_col_size,
    u8 *arr,

    const u8 palette[256],

    const int offset_row,
    const int offset_col)
{
    if (offset_row < 0 || offset_col < 0)
    {
        return;
    }

    for (int row = 0; row < min(arr_row_size - offset_row, sprite_row_size); row++)
    {
        for (int col = 0; col < min(arr_col_size - offset_col, sprite_col_size); col++)
        {
            if (sprite[(row * sprite_col_size) + col] != 0)
            {
                arr[((row + offset_row) * (arr_col_size * 3)) + ((col + offset_col) * 3) + 0] = 255;
                arr[((row + offset_row) * (arr_col_size * 3)) + ((col + offset_col) * 3) + 1] = 255;
                arr[((row + offset_row) * (arr_col_size * 3)) + ((col + offset_col) * 3) + 2] = 255;
            }
        }
    }
}



void get_diffs(
    u16 *result,
    const u8 *arr_1,
    const u8 *arr_2,
    const int size
)
{
    for (int i = 0; i < size; i++)
    {
        result[i] = (arr_1[i] != arr_2[i]);
    }
}

void get_window_sums_(
    const int arr_row_size,
    const int arr_col_size,
    const int window_row_size,
    const int window_col_size,
    u16 *window_sums_temp,
    u16 *window_sums
)
{
    const int size = arr_row_size * arr_col_size;
    const int window_sums_col_size = arr_col_size - (window_col_size - 1);

    // Loop through each row
    for (int row = 0; row < arr_row_size; row++)
    {
        // Calculate cumulative sum for this row
        for (int col = 1; col < arr_col_size; col++)
        {
            window_sums_temp[(row * arr_col_size) + col] += window_sums_temp[(row * arr_col_size) + col - 1];
        }

        // Calculate window sums for this row
        for (int col = arr_col_size - 1; col >= window_col_size; col--)
        {
            window_sums_temp[(row * arr_col_size) + col] -= window_sums_temp[(row * arr_col_size) + col - window_col_size];
        }
    }

    // Calculate cumulative sums for each column
    for (int row = 1; row < arr_row_size; row++)
    {
        for (int col = window_col_size - 1; col < arr_col_size; col++)
        {
            window_sums_temp[(row * arr_col_size) + col] += window_sums_temp[((row - 1) * arr_col_size) + col];
        }
    }

    // Calculate sums for each window
    for (int row = arr_row_size - 1; row >= window_row_size; row--)
    {
        for (int col = arr_col_size; col >= window_col_size - 1; col--)
        {
            window_sums[((row - window_row_size + 1) * window_sums_col_size) + col - window_col_size + 1] = window_sums_temp[(row * arr_col_size) + col] - window_sums_temp[((row - window_row_size) * arr_col_size) + col];
        }
    }

    // Add top row
    for (int col = 0; col < arr_col_size - (window_col_size - 1); col++)
    {
        window_sums[col] = window_sums_temp[((window_row_size - 1) * arr_col_size) + col + window_col_size - 1];
    }

    // Add left column
    for (int row = 1; row < arr_row_size - (window_row_size - 1); row++)
    {
        window_sums[row * window_sums_col_size] = window_sums_temp[((row + window_row_size - 1) * arr_col_size) + window_col_size - 1] - window_sums_temp[((row - 1) * arr_col_size) + window_col_size - 1];
    }
}

void match_sprites(
    int num_rows,
    int num_cols,
    u8 *image,
    const u8 *image_transformed_1,
    const u8 *image_transformed_2,
    int *match_rows,
    int *match_cols,
    int *match_sprite_ids,
    int *match_palette_ids,
    int *num_matches_found,
    const int max_num_matches,
    const int is_debug)
{
    clock_t begin = clock();

    const int size = num_rows * num_cols;

    u16 *diffs = malloc(size * sizeof(u16));
    get_diffs(diffs, image_transformed_1, image_transformed_2, size);

    u16 *window_sums = malloc((num_rows - (gl.window_row_size - 1)) * (num_cols - (gl.window_col_size - 1)) * sizeof(u16));
    get_window_sums_(num_rows, num_cols, gl.window_row_size, gl.window_col_size, diffs, window_sums);

    const int num_window_rows = (num_rows - (gl.window_row_size - 1));
    const int num_window_cols = (num_cols - (gl.window_col_size - 1));

    Match *matches = malloc(max_num_matches * sizeof(Match));
    int num_matches = 0;

    int *matching_keys = malloc(max_num_matches * sizeof(int));

    Point point;
    u8 values[WINDOW_SIZE];

    // TODO Some of the inner loops should be moved to separate functions
    for (int row = 0; row < num_window_rows; row++)
    {
        for (int col = 0; col < num_window_cols; col++)
        {

            int window_sums_index = (row * (num_cols - (gl.window_col_size - 1))) + col;

            if (window_sums[window_sums_index] == 0)
            {
                continue;
            }

            point.row = row;
            point.col = col;

            copy_range(values, num_rows, num_cols, image_transformed_2, point, gl.window_row_size, gl.window_col_size);

            int num_element_matches = find_element(gl.hash_map, gl.data, values, gl.palettes, max_num_matches, matching_keys);

            for (int i = 0; i < num_element_matches; i++)
            {
                int key = matching_keys[i];

                u32 data_id = (gl.hash_map.data[key] >> 11);
                u32 palette_id = (gl.hash_map.data[key] & 0x7FF);
                int sprite_id = gl.data[data_id].sprite_id;

                int offset_row = (int)(row - gl.data[data_id].point.row);
                int offset_col = (int)(col - gl.data[data_id].point.col);

                int offset_row_ = offset_row + gl.sprites[sprite_id].row_offset;
                int offset_col_ = offset_col + gl.sprites[sprite_id].col_offset;

                // TODO This should be moved to a separate function
                int is_sprite_already_found = 0;
                for (int j = 0; j < num_matches; j++)
                {
                    if (
                        abs((offset_row_ - matches[j].row)) < 5 &&
                        abs((offset_col_ - matches[j].col)) < 5)
                    {
                        is_sprite_already_found = 1;
                    }
                }

                if (is_sprite_already_found == 0)
                {
                    int is_sprite_valid = validate_sprite(
                        gl.sprites[sprite_id].row_size,
                        gl.sprites[sprite_id].col_size,
                        gl.sprites[sprite_id].data,
                        num_rows,
                        num_cols,
                        image_transformed_2,
                        gl.palettes + (palette_id * 256),
                        offset_row,
                        offset_col);

                    if (is_sprite_valid)
                    {
                        matches[num_matches].row = offset_row_;
                        matches[num_matches].col = offset_col_;
                        matches[num_matches].sprite_id = sprite_id;
                        matches[num_matches].palette_id = palette_id;

                        match_rows[num_matches] = offset_row;
                        match_cols[num_matches] = offset_col;
                        match_sprite_ids[num_matches] = sprite_id;
                        match_palette_ids[num_matches] = palette_id;

                        num_matches++;

                        num_matches_found[0] = num_matches;

                        if (num_matches == max_num_matches)
                        {
                            free(matches);
                            free(matching_keys);
                            free(diffs);
                            free(window_sums);
                            return;
                        }

                        if (is_debug)
                        {
                            draw_sprite(
                                gl.sprites[sprite_id].row_size,
                                gl.sprites[sprite_id].col_size,
                                gl.sprites[sprite_id].data,
                                num_rows,
                                num_cols,
                                image,
                                gl.palettes + (palette_id * 256),
                                row - gl.data[data_id].point.row,
                                col - gl.data[data_id].point.col);

                            // Draw matching windows
                            for (int row_ = row; row_ < row + gl.window_row_size; row_++)
                            {
                                for (int col_ = col; col_ < col + gl.window_col_size; col_++)
                                {
                                    image[(row_ * num_cols * 3) + (col_ * 3) + 0] = 255;
                                    image[(row_ * num_cols * 3) + (col_ * 3) + 1] = 40;
                                    image[(row_ * num_cols * 3) + (col_ * 3) + 2] = 179;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    free(matches);
    free(matching_keys);
    free(diffs);
    free(window_sums);

    if (is_debug)
    {
        printf("%s: %f\n", "runtime", (double)(clock() - begin) / CLOCKS_PER_SEC);
    }
}

Slice get_key(char *data, int size, int c)
{
    Slice slice = {0};

    while (c < size)
    {
        if (data[c] == '"')
        {
            if (slice.start == 0)
            {
                slice.start = c + 1;
            }
            else
            {
                slice.end = c;
                break;
            }
        }
        c++;
    }

    return slice;
}

int is_integer(char c)
{
    return (c >= '0' && c <= '9');
}

int chars_to_int(char *text, Slice slice)
{
    int num = 0;
    int multiple = 1;

    for (int i = slice.end - 1; i >= slice.start; i--)
    {
        num += (text[i] - '0') * multiple;
        multiple *= 10;
    }

    return num;
}

Slice get_next_integer(char *data, int size, int c)
{
    Slice slice = {0};

    while (c < size)
    {
        if (is_integer(data[c]) && slice.start == 0)
        {
            slice.start = c;
        }
        else if (is_integer(data[c]) == 0 && slice.start != 0)
        {
            slice.end = c;
            break;
        }
        c++;
    }

    return slice;
}

int get_next_element_in_same_level(char *data, int size, Counts *counts, int c)
{
    while (c < size)
    {
        if (data[c] == '{' || data[c] == '[')
        {
            counts->open++;
        }
        else if (data[c] == '}' || data[c] == ']')
        {
            counts->close++;
        }
        else if (data[c] == ',' && counts->open == (counts->close + 1))
        {
            break;
        }
        c++;
    }

    return c;
}

void print_slice(char *data, Slice slice)
{
    for (int i = slice.start; i < slice.end; i++)
    {
        printf("%c", data[i]);
    }
    printf("\n");
}

int does_strings_match(char *a, char *b, int length)
{
    for (int i = 0; i < length; i++)
    {
        if (a[i] != b[i])
        {
            return 0;
        }
    }

    return 1;
}

int find_matching_key(char *data, int size, char *key_to_find)
{
    int c = 0;
    Slice key;
    Counts counts = {1, 0};

    while (1)
    {
        key = get_key(data, size, c);

        if (key.start == 0) // If we couldn't find a next key.
        {
            break;
        }

        if (does_strings_match(key_to_find, data + key.start, key.end - key.start))
        {
            return key.end;
        }

        c = get_next_element_in_same_level(data, size, &counts, key.end);
    }

    return 0;
}

int get_first_index(char *data, int size, int c)
{
    while (c < size)
    {
        if (data[c] == '{' || data[c] == '[')
        {
            return c + 1;
        }

        c++;
    }
    return c;
}

int count_keys(char *data, int size, int c)
{
    int count = 1;

    Counts counts = {0};

    while (c < size)
    {
        if (data[c] == '{' || data[c] == '[')
        {
            counts.open++;
        }
        else if (data[c] == '}' || data[c] == ']')
        {
            counts.close++;
        }
        else if (data[c] == ',')
        {
            if (counts.open == (counts.close + 1))
            {
                count++;
            }
            else if (counts.open == counts.close)
            {
                break;
            }
        }
        c++;
    }

    return count;
}

int count_list_elements(char *data, int size, int c)
{
    int count = 1;
    while (c < size)
    {
        if (data[c] == ']')
        {
            break;
        }
        else if (data[c] == ',')
        {
            count++;
        }

        c++;
    }

    return count;
}

void create_path(char *path, char *folder, char *monster, char *composit, char *file)
{
    int c = 0;

    for (int i = 0; i < strlen(folder); i++)
    {
        path[c++] = folder[i];
    }
    path[c++] = '/';

    path[c++] = monster[0];
    path[c++] = monster[1];
    path[c++] = '/';

    path[c++] = composit[0];
    path[c++] = composit[1];
    path[c++] = '/';

    for (int i = 0; i < strlen(file); i++)
    {
        path[c++] = file[i];
    }

    path[c] = '\0';
}

void create_path_palshift(char *path, char *folder, char *monster, char *file)
{
    int c = 0;

    for (int i = 0; i < strlen(folder); i++)
    {
        path[c++] = folder[i];
    }
    path[c++] = '/';

    path[c++] = monster[0];
    path[c++] = monster[1];
    path[c++] = '/';

    for (int i = 0; i < strlen(file); i++)
    {
        path[c++] = file[i];
    }

    path[c] = '\0';
}

int has_special_palshift(int *palshift_ids, int num_palshifts)
{
    for (int i = 0; i < num_palshifts; i++)
    {
        if (palshift_ids[i] != 0)
        {
            return 1;
        }
    }

    return 0;
}

void parse_json(char *data, int size, char *zone, char *monster_folder_path, _File act_palettes, _File unique_champion_palettes)
{
    int zone_key_index = find_matching_key(data, size, zone);
    if (zone_key_index == 0)
    {
        printf("Could not find zone %s in json file. Exiting program", zone);
        exit(1);
    }

    int num_monsters = count_keys(data, size, zone_key_index);
    // printf("num_monsters: %d\n", num_monsters);

    u8 **palshifts = malloc(num_monsters * sizeof(u8 *));
    int *num_palshifts = malloc(num_monsters * sizeof(int));
    int **palshift_ids = malloc(num_monsters * sizeof(int *));
    int *num_composits = malloc(num_monsters * sizeof(int));
    u8 ***composits = malloc(num_monsters * sizeof(u8 **));

    int c = zone_key_index + 1;
    Slice key;
    Counts counts = {1, 0};
    char file_path[256];

    for (int i = 0; i < num_monsters; i++)
    {
        key = get_key(data, size, c);

        int monster_list_index = get_first_index(data, size, key.end);
        int composit_list_index = get_first_index(data, size, monster_list_index);
        int palshift_list_index = get_first_index(data, size, composit_list_index);

        num_composits[i] = count_list_elements(data, size, composit_list_index);
        num_palshifts[i] = count_list_elements(data, size, palshift_list_index);

        u8 **composits_ = malloc(num_composits[i] * sizeof(u8 *));
        composits[i] = composits_;

        int *palshift_ids_ = malloc(num_palshifts[i] * sizeof(int));
        palshift_ids[i] = palshift_ids_;

        for (int j = 0; j < num_composits[i]; j++)
        {
            Slice composit = get_key(data, size, composit_list_index);
            composit_list_index = composit.end + 1;

            create_path(file_path, monster_folder_path, data + key.start, data + composit.start, "data.dat");
            composits[i][j] = read_file(file_path).data;
        }

        for (int j = 0; j < num_palshifts[i]; j++)
        {
            Slice integer = get_next_integer(data, size, palshift_list_index);
            palshift_list_index = integer.end + 1;

            palshift_ids[i][j] = chars_to_int(data, integer);
        }

        if (has_special_palshift(palshift_ids[i], num_palshifts[i]))
        {
            create_path_palshift(file_path, monster_folder_path, data + key.start, "palshift.dat");
            palshifts[i] = read_file(file_path).data;
        }

        c = get_next_element_in_same_level(data, size, &counts, key.end);
    }

    int **num_sprites_per_composit = malloc(num_monsters * sizeof(int *));
    for (int i = 0; i < num_monsters; i++)
    {
        int *num_sprites_per_composit_ = malloc(num_composits[i] * sizeof(int));
        num_sprites_per_composit[i] = num_sprites_per_composit_;
    }

    Sprite_Group sprite_group;
    sprite_group.monster_composits = composits;
    sprite_group.monster_palshift_ids = palshift_ids;
    sprite_group.monster_palshifts = palshifts;
    sprite_group.num_composits_per_monster = num_composits;
    sprite_group.num_monsters = num_monsters;
    sprite_group.num_palshifts_per_monster = num_palshifts;
    sprite_group.num_sprites_per_monster = malloc(num_monsters * sizeof(int));
    sprite_group.num_sprites_per_composit = num_sprites_per_composit;

    set_data(sprite_group, act_palettes.data, unique_champion_palettes.data);

    free(act_palettes.data);
    free(unique_champion_palettes.data);
}

char int_to_char(int num)
{
    return num + '0';
}

int sum_string_lengths(char **strings, int num_strings)
{
    int len = 0;

    for (int i = 0; i < num_strings; i++)
    {
        len += strlen(strings[i]);
    }

    return len;
}

int get_path_length(char **paths, int num_paths)
{
    int string_lengths = sum_string_lengths(paths, num_paths);
    int separator_lengths = num_paths;
    int null_terminator_length = 1;

    return string_lengths + separator_lengths + null_terminator_length;
}

char *join_paths(char **paths, int num_paths)
{
    int len = get_path_length(paths, num_paths);
    char *path = malloc(len * sizeof(char));

    int c = 0;

    for (int i = 0; i < num_paths; i++)
    {
        for (int j = 0; j < strlen(paths[i]); j++)
        {
            path[c++] = paths[i][j];
        }

        // Add separator if this is not the last item
        if (i < num_paths - 1)
        {
            path[c++] = '/';
        }
    }

    path[c] = '\0';

    return path;
}

char *combine_strings(char *a, char b)
{
    int len_a = strlen(a);
    int len_b = 1;
    int len_total = len_a + len_b + 1;

    char *string = malloc(len_total * sizeof(char));

    int c = 0;

    for (int i = 0; i < len_a; i++)
    {
        string[c++] = a[i];
    }

    string[c++] = b;

    string[c] = '\0';

    return string;
}

void _set_data(int act_number, char *root_folder, char *zone)
{
    char *monster_folder_path = join_paths((char *[]){root_folder, "initialization_data", "monster_sprites"}, 3);
    char *act_palette_path = join_paths((char *[]){root_folder, "initialization_data", "palette", combine_strings("ACT", int_to_char(act_number)), "Pal.PL2"}, 5);
    char *unique_champion_palettes_path = join_paths((char *[]){root_folder, "initialization_data", "RandTransforms.dat"}, 3);
    char *zones_monsters_path = join_paths((char *[]){root_folder, "initialization_data", "zones_monsters.json"}, 3);

    _File act_palettes = read_file(act_palette_path);
    _File unique_champion_palettes = read_file(unique_champion_palettes_path);

    _File zones_monsters = read_file(zones_monsters_path);
    parse_json((char *)zones_monsters.data, zones_monsters.size, zone, monster_folder_path, act_palettes, unique_champion_palettes);

    free(monster_folder_path);
    free(act_palette_path);
    free(unique_champion_palettes_path);
    free(zones_monsters_path);
    
}

int main()
{
    _set_data(1, "hello", "act_1_wilderness_1");
    return 0;
}