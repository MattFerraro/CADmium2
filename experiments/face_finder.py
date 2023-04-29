import math


class Point:
    def __init__(self, x, y, id):
        self.x = x
        self.y = y
        # id just gives us something to sort by
        self.id = id

    def __repr__(self):
        return f"{self.x:.02f}, {self.y:.02f}"

    def clone(self):
        return Point(self.x, self.y, self.id)

    def equals(self, other):
        return self.x == other.x and self.y == other.y and self.id == other.id


class Line:
    def __init__(self, start, end):
        self.start = start
        self.end = end

    def clone(self):
        return Line(self.start.clone(), self.end.clone())

    def angle(self):
        dx = self.end.x - self.start.x
        dy = self.end.y - self.start.y
        return math.atan2(dy, dx)

    def angle_degrees(self):
        return self.angle() * 180 / math.pi

    def reverse(self):
        return Line(self.end, self.start)

    def __repr__(self):
        return f"line from {self.start.id} to {self.end.id} {self.angle_degrees():.02f}"


class Arc:
    def __init__(self, start, end, transit):
        self.start = start
        self.end = end
        self.transit = transit

    def clone(self):
        return Arc(self.start.clone(), self.end.clone(), self.transit.clone())

    def reverse(self):
        return Arc(self.end.clone(), self.start.clone(), self.transit.clone())

    def angle(self):
        start_transit_dx = self.transit.x - self.start.x_dx
        start_transit_dy = self.transit.y - self.start.y
        start_transit_m = start_transit_dy / start_transit_dx
        start_transit_midpoint_x = (self.transit.x + self.start.x) / 2
        start_transit_midpoint_y = (self.transit.y + self.start.y) / 2
        start_transit_m_f = -1 / start_transit_m

        transit_end_dx = self.end.x - self.transit.x
        transit_end_dy = self.end.y - self.transit.y
        transit_end_m = transit_end_dy / transit_end_dx
        transit_end_midpoint_x = (self.transit.x + self.end.x) / 2
        transit_end_midpoint_y = (self.transit.y + self.end.y) / 2
        transit_end_m_f = -1 / transit_end_m

    def angle_degrees(self):
        return self.angle() * 180 / math.pi


class Segment:
    def __init__(self, thing):
        self.thing = thing
        self.used_forward = False
        self.used_backward = False

    def line_or_arc(self):
        return self.thing

    def reverse(self):
        return Segment(self.thing.reverse())

    def __repr__(self):
        return f"{str(self.thing)}"


class Sketch:
    def __init__(self, segments):
        self.segments = segments


class Ring:
    def __init__(self, segments):
        self.segments = segments


class Face:
    def __init__(self, rings):
        self.rings = rings


class Connection:
    def __init__(self, other_point, segment):
        self.other_point = other_point
        self.segment = segment


def find_angle(a, b, c):
    # point b is the center
    return math.atan2(c.y - b.y, c.x - b.x) - \
        math.atan2(a.y - b.y, a.x - b.x)


def try_find_ring(segment, points, connections):
    # segment is the starting segment
    # correct_order is boolean. If False, invert this segment first
    print("Trying to find ring from segment:", segment)
    line_or_arc = segment.line_or_arc()

    point_0 = line_or_arc.start
    point_1 = line_or_arc.end

    print(f"from {point_0.id} to {point_1.id}")
    potential_connections = connections[point_1.id]
    # print(potential_connections)
    # remove the one that is just this segment in reverse

    # for potential_next_segment in potential_connections:
    # print(f"\tPotentially: {potential_next_segment}, angle {compute_angle()}")

    # for each remaining, compute the angles formed between this segment and
    # the one in question. Which one turns our path CCW the hardest?
    # choose that one, step to it, then


def find_faces(sketch):
    # First remove the degenerate points which can't form faces
    sketch = clean_sketch(sketch)

    # Second build a map of the points and their connections
    points, connections = compute_points_and_connections(sketch)
    # Points:  {'A': 0.00, 0.00, 'B': 1.00, 0.00, 'C': 0.00, 1.00}
    # Connections: {
    #   'A': [('B', line from A to B 0.00, True), ('C', line from C to A - 90.00, False)],
    #   'B': [('A', line from A to B 0.00, False), ('C', line from B to C 135.00, True)],
    #   'C': [('B', line from B to C 135.00, False), ('A', line from C to A - 90.00, True)]
    # }

    rings = []
    for segment in sketch.segments:
        new_ring = try_find_ring(segment, points, connections)
        if new_ring:
            rings.append(new_ring)

        new_ring = try_find_ring(segment.reverse(), points, connections)
        if new_ring:
            rings.append(new_ring)

    faces = []
    return faces


def clean_sketch(sketch):
    while True:
        points, connections = compute_points_and_connections(sketch)
        degens = identify_degenerate_points(points, connections)
        if len(degens) == 0:
            break
        sketch = prune_degenerate(sketch, degens)
    return sketch


def prune_degenerate(sketch, degens):
    to_keep = []
    for segment in sketch.segments:
        if segment.thing.start.id in degens:
            continue
        elif segment.thing.end.id in degens:
            continue
        else:
            to_keep.append(segment)

    return Sketch(to_keep)


def identify_degenerate_points(points, connections):
    degenerate_points = []
    for point_id, connection_list in connections.items():
        # print(point_id, connection_list)
        if len(connection_list) <= 1:
            # print("\tWHOA - not enough connections")
            degenerate_points.append(point_id)
    return degenerate_points


def compute_points_and_connections(sketch):
    points = {}
    connections = {}
    for segment in sketch.segments:
        line_or_arc = segment.line_or_arc()
        points[line_or_arc.start.id] = line_or_arc.start
        points[line_or_arc.end.id] = line_or_arc.end

        if not line_or_arc.start.id in connections:
            connections[line_or_arc.start.id] = []
        connections_list_start = connections[line_or_arc.start.id]
        connections_list_start.append((line_or_arc.end.id, line_or_arc, True))

        if not line_or_arc.end.id in connections:
            connections[line_or_arc.end.id] = []
        connections_list_end = connections[line_or_arc.end.id]
        connections_list_end.append((line_or_arc.start.id, line_or_arc, False))

    return points, connections


def test_degenerate():
    # nothing at all

    # just one point
    # two points unconnected
    # two points connected
    # three points, only 2 segments
    # four points, only 2 segments
    pass


def test_simplest_shapes():
    # three points, simply connected in a triangle
    A = Point(0.0, 0.0, "A")
    B = Point(1.0, 0.0, "B")
    C = Point(0.0, 1.0, "C")
    line_ab = Line(A, B)
    line_bc = Line(B, C)
    line_ca = Line(C, A)

    # these extras get pruned
    D = Point(-1.0, 0.0, "D")
    E = Point(-2.0, 0.0, "E")
    line_ad = Line(A, D)
    line_de = Line(D, E)

    segments = [Segment(l)
                for l in [line_ab, line_bc, line_ca, line_ad, line_de]]
    sketch = Sketch(segments)
    faces = find_faces(sketch)

    # matt_ferraro@planet.com

    # assert len(faces) == 1
    # print(faces[0])

    # four points, a simple square
    # eight points, an octogon
    pass


def main():
    test_simplest_shapes()


if __name__ == '__main__':
    main()
