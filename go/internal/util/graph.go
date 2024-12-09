package util

// DAGraph is a directed acyclic graph.
type DAGraph struct {
	Adjacencies map[int][]int
}

// NewDAGraph creates a new directed acyclic graph.
func NewDAGraph() *DAGraph {
	return &DAGraph{
		Adjacencies: make(map[int][]int),
	}
}

// AddEdge adds an edge to the graph.
func (g *DAGraph) AddEdge(from, to int) {
	g.Adjacencies[from] = append(g.Adjacencies[from], to)
}

// HasCycle returns true if the graph has a cycle.
func (g *DAGraph) HasCycle() bool {
	done := make(map[int]bool)
	seen := make(map[int]bool)

	stack := make([]int, 0)

	for key, _ := range g.Adjacencies {
		stack = append(stack, key)
	}
	
	for len(stack) > 0 {
		node := stack[len(stack)-1]

		if seen[node] {
			stack = stack[:len(stack)-1]
			done[node] = true
			continue
		} else if done[node] {
			return true
		}

		// mark node as seen
		seen[node] = true
	
		stack = append(stack, g.Adjacencies[node]...)
	}

	return false
}

// TopologicalSort returns a topological sort of the graph. It assumes the graph is a DAG.
func (g *DAGraph) TopologicalSort() []int {
	seen := make(map[int]bool)
	order := make([]int, 0)

	var dfs func(int)
	dfs = func(node int) {
		if seen[node] {
			return
		}

		seen[node] = true

		for _, neighbor := range g.Adjacencies[node] {
			dfs(neighbor)
		}

		order = append([]int{node}, order...)
	}
	
	for node, _ := range g.Adjacencies {
		dfs(node)
	}

	return order
}
