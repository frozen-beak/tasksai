"""\nA* Search Algorithm Implementation.\n\nThis module implements the A* search algorithm to find the shortest path between two points on a grid.\n"""

import heapq

def heuristic(a, b):
    """
    Calculate the Manhattan distance heuristic between two points.\n
    This heuristic estimates the cost to reach the goal from a given node.

    Args:
        a (tuple): The first point (x, y).
        b (tuple): The second point (x, y).

    Returns:
        int: The Manhattan distance between the two points.
    """
    (x1, y1) = a
    (x2, y2) = b
    return abs(x1 - x2) + abs(y1 - y2)


def get_neighbors(node, grid):
    """
    Get the valid neighbors of a node in the grid.

    A neighbor is valid if it is within the grid boundaries and is not an obstacle (value of 0).

    Args:
        node (tuple): The current node (x, y).
        grid (list[list[int]]): The grid representing the search space.

    Returns:
        list[tuple]: A list of valid neighbor nodes (x, y).
    """
    neighbors = []
    directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]  # Possible movement directions: right, down, left, up
    for dx, dy in directions:
        x2, y2 = node[0] + dx, node[1] + dy
        if 0 <= x2 < len(grid) and 0 <= y2 < len(grid[0]) and grid[x2][y2] == 0:
            neighbors.append((x2, y2))
    return neighbors

def reconstruct_path(came_from, current):
    """
    Reconstruct the path from the goal node to the start node.

    This function traces back the path by following the 'came_from' dictionary.

    Args:
        came_from (dict): A dictionary mapping a node to its predecessor in the path.
        current (tuple): The current node (the goal node).

    Returns:
        list[tuple]: The reconstructed path from start to goal.
    """
    path = [current]
    while current in came_from:
        current = came_from[current]
        path.append(current)
    path.reverse()
    return path

def a_star_search(grid, start, goal):
    """
    Perform A* search to find the shortest path from start to goal on a grid.

    Args:
        grid (list[list[int]]): A 2D grid where 0 represents a traversable cell and 1 represents an obstacle.
        start (tuple): The starting node (x, y).
        goal (tuple): The goal node (x, y).

    Returns:
        list[tuple]: The shortest path from start to goal, or an empty list if no path is found.
    """
    open_set = []  # Priority queue for nodes to evaluate
    heapq.heappush(open_set, (0, 0, start))  # (f_score, tie-breaker count, node)
    came_from = {}  # Dictionary to reconstruct the path

    g_score = {start: 0}  # Cost from start to node
    f_score = {start: heuristic(start, goal)}  # Estimated cost from start to goal through node

    count = 0  # Tie-breaker to ensure consistent ordering in the priority queue

    while open_set:
        current_f, _, current = heapq.heappop(open_set)  # Get the node with the lowest f_score

        if current == goal:
            return reconstruct_path(came_from, current)

        for neighbor in get_neighbors(current, grid):
            tentative_g_score = g_score[current] + 1  # Cost from start to neighbor through current

            if tentative_g_score < g_score.get(neighbor, float('inf')):
                # This path to neighbor is better than any previous one.
                came_from[neighbor] = current
                g_score[neighbor] = tentative_g_score
                f_score[neighbor] = tentative_g_score + heuristic(neighbor, goal)
                count += 1  # Increment tie-breaker
                heapq.heappush(open_set, (f_score[neighbor], count, neighbor))

    return []  # No path found

# Example Usage:
grid = [
    [0, 0, 0, 0, 0],
    [0, 1, 1, 1, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 0, 1, 0],
    [0, 0, 0, 1, 0],
]
start = (0, 0)  # Start node
goal = (4, 4)  # Goal node

path = a_star_search(grid, start, goal)
if path:
    print("Path found:")
    for step in path:
        print(step)
else:
    print("No path found.")