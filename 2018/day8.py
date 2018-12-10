# https://adventofcode.com/2018/day/8

def read_node(input):
    # Header
    num_child = input[0]
    num_meta = input[1]
    num_read = 2
    
    # Children
    children = []
    for _ in range(num_child):
        child, n = read_node(input[num_read:])
        num_read += n
        children.append(child)

    # Metadata
    metadata = input[num_read:num_read+num_meta]
    num_read += num_meta

    # Node
    return (children, metadata), num_read

def read_tree():
    with open('day8.txt') as f:
        input = [int(n) for n in f.read().strip().split()]
    tree, n = read_node(input)
    assert n == len(input)
    return tree

def sum_meta(node):
    return sum(sum_meta(c) for c in node[0]) + sum(node[1])

def node_value(node):
    return sum(node[1]) if len(node[0]) == 0 else sum(node_value(node[0][m-1]) for m in node[1] if 0 < m <= len(node[0]))

tree = read_tree()
print(sum_meta(tree))
print(node_value(tree))