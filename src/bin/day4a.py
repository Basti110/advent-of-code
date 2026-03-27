
def check_x(xmas_matrix, i, j):
    directions = [
        [1, 1],
        [-1, -1]
    ]
    counter = 0
    for direction in directions:
        i_temp = i
        j_temp = j
        for x in ['X', 'M', 'A', 'S']:
            if xmas_matrix[i_temp][j_temp] != x:
                break
            i_temp += direction[0]
            j_temp += direction[1]
        counter += 1
    return counter


def main(filepath: str):
    lines = []

    with open(filepath, 'r', encoding='utf-8') as file:
        lines = file.readlines()
        lines = [line.strip() for line in lines]

    # xmas_matrix = [[i for i in line] for line in lines]
    xmas_matrix = []
    for line in lines:
        x_mas_line = []
        for i in line:
            x_mas_line.append(i)
        xmas_matrix.append(x_mas_line)

    for i in range(len(xmas_matrix)):
        for j in range(len(xmas_matrix[i])):
            if xmas_matrix[i][j] == 'X':
                check_x(xmas_matrix, i, j)




# Example usage:
if __name__ == "__main__":
    filepath = "./input/day4a.txt"  # Replace with your file path
    main(filepath)
