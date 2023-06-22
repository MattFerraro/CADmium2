function Box(props) {
    // This reference gives us direct access to the THREE.Mesh object
    const ref = useRef()
    // Hold state for hovered and clicked events
    const [hovered, hover] = useState(false)
    const [clicked, click] = useState(false)
    // Subscribe this component to the render-loop, rotate the mesh every frame
    useFrame((state, delta) => (ref.current.rotation.x += delta))
    // Return the view, these are regular Threejs elements expressed in JSX
    return (
        <mesh
            {...props}
            ref={ref}
            scale={clicked ? 1.5 : 1}
            onClick={(event) => click(!clicked)}
            onPointerOver={(event) => hover(true)}
            onPointerOut={(event) => hover(false)}
        >
            <boxGeometry args={[1, 1, 1]} />
            <meshStandardMaterial color={hovered ? 'hotpink' : 'orange'} />
        </mesh>
    )
}

function Solid({ mesh, style }) {
    const ref = useRef()
    // useHelper(ref, VertexNormalsHelper, .3, "green");
    const [hovered, hover] = useState(false)
    const positions = new Float32Array(
        mesh.vertices.flatMap((v) => [v.x, v.y, v.z])
    )
    const normals = new Float32Array(mesh.normals.flatMap((v) => [v.x, v.y, v.z]))
    const indices = new Uint16Array(mesh.indices)

    return (
        <mesh
            ref={ref}
            onPointerOver={(event) => hover(false)}
            onPointerOut={(event) => hover(false)}
        >
            <bufferGeometry attach="geometry">
                <bufferAttribute
                    attach="attributes-position"
                    array={positions}
                    count={positions.length / 3}
                    itemSize={3}
                />
                <bufferAttribute
                    attach="attributes-normal"
                    array={normals}
                    count={normals.length / 3}
                    itemSize={3}
                />
                <bufferAttribute
                    attach="index"
                    array={indices}
                    count={indices.length}
                    itemSize={1}
                />
            </bufferGeometry>
            {style === 'solid' && (
                <meshStandardMaterial
                    metalness={0.75}
                    roughness={0.17}
                    color={hovered ? 'hotpink' : '#006B3C'}
                    side={THREE.DoubleSide}
                />
            )}

            {style === 'plane' && (
                <meshStandardMaterial
                    color="#006B3C"
                    opacity={0.1}
                    transparent
                    side={THREE.DoubleSide}
                />
            )}
        </mesh>
    )
}
