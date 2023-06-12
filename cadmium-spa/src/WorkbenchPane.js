import './App.css'
import React, { useCallback, useEffect, useRef, useState } from 'react'
import { Canvas, useFrame } from '@react-three/fiber'
import { CameraControls, Environment, useHelper, Text } from '@react-three/drei'
import * as THREE from 'three'
import studio_2_1k from './images/studio_2_1k.hdr'
// import { VertexNormalsHelper } from "three/examples/jsm/helpers/VertexNormalsHelper";

import { useThree } from '@react-three/fiber'

function WorkbenchPane({ workbenchView }) {

  let parts = null;
  if (workbenchView) {
    parts = workbenchView.solids.map((solid) => solid.get("solid").get_mesh());
  }
  let planes = null;
  if (workbenchView) {
    planes = workbenchView.planes;
  }
  const mouseConfig = useCallback((camControls) => {
    if (camControls !== null) {
      camControls.mouseButtons.middle = 8
      camControls.mouseButtons.wheel = 8
      camControls.mouseButtons.right = 1
      camControls.mouseButtons.left = 0
    }
    //  NONE: 0,
    // 	ROTATE: 1,
    // 	TRUCK: 2,
    // 	OFFSET: 4,
    // 	DOLLY: 8,
    // see https://github.com/yomotsu/camera-controls/blob/29eac5b50e69f0cf6792b8c3c12f5c86ad621222/src/types.ts
  }, [])

  const overallScale = 100;

  return (
    <Canvas camera={{ far: 10000, fov: 35, position: [1 * overallScale, -1 * overallScale, 1 * overallScale], up: [0, 0, 1] }} style={{ height: '100%' }}>
      <Environment files={studio_2_1k} />

      <CameraControls ref={mouseConfig} />
      <ambientLight />
      <pointLight position={[5 * overallScale, -5 * overallScale, 5 * overallScale]} />
      <pointLight position={[-5 * overallScale, 5 * overallScale, 5 * overallScale]} />

      {parts && parts.map((part, index) => {
        return <Part key={index} mesh={part}></Part>
      })}

      {planes && planes.map((plane, index) => {
        return <Plane key={index} plane={plane}></Plane>
      })}
      <axesHelper></axesHelper>
    </Canvas>
  )
}

function Part({ mesh }) {
  return <>
    <Solid mesh={mesh} style="solid"></Solid>
    <Wireframe mesh={mesh}></Wireframe>
  </>
}

function Solid({ mesh, style }) {
  const ref = useRef()
  // useHelper(ref, VertexNormalsHelper, .3, "green");
  const [hovered, hover] = useState(false)
  const positions = new Float32Array(mesh.vertices.flatMap((v) => [v.x, v.y, v.z]));
  const normals = new Float32Array(mesh.normals.flatMap((v) => [v.x, v.y, v.z]));
  const indices = new Uint16Array(mesh.indices);

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
      {style === "solid" && <meshStandardMaterial
        metalness={0.75}
        roughness={0.17}
        color={hovered ? 'hotpink' : '#e30022'}
        side={THREE.DoubleSide}
      />}

      {style === "plane" && <meshStandardMaterial
        color="#ff0000" opacity={0.1} transparent
        side={THREE.DoubleSide}
      />}


      {/* <meshNormalMaterial
        color={hovered ? 'hotpink' : '#5cffb7'}
        side={THREE.DoubleSide}
      /> */}
      {/* <VertexNormalsHelper args={[ref, 0.2, 0x00ff00, 1]}></VertexNormalsHelper> */}
    </mesh>
  )
}

function Wireframe({ mesh }) {
  const ref = useRef()
  const [hovered, hover] = useState(false)
  const positions = new Float32Array(mesh.vertices.flatMap((v) => [v.x, v.y, v.z]));
  const normals = new Float32Array(mesh.normals.flatMap((v) => [v.x, v.y, v.z]));
  const indices = new Uint16Array(mesh.indices);

  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute('position', new THREE.BufferAttribute(positions, 3));
  geometry.setAttribute('normal', new THREE.BufferAttribute(normals, 3));
  geometry.setIndex(new THREE.BufferAttribute(indices, 1));
  const edges = new THREE.EdgesGeometry(geometry, 5);
  const line = new THREE.LineSegments(edges, new THREE.LineBasicMaterial({ color: 0x000000 }));

  return (
    <mesh
      ref={ref}
      onPointerOver={(event) => hover(false)}
      onPointerOut={(event) => hover(false)}
    >
      <lineSegments geometry={edges} material={line.material} />
      <meshStandardMaterial
        polygonOffset={true}
        polygonOffsetFactor={1} // positive value pushes polygon further away
        polygonOffsetUnits={1}
        color={hovered ? 'hotpink' : '#006B3C'}
        side={THREE.DoubleSide}
      />
    </mesh>
  )
}


function Plane({ plane }) {
  const actualPlane = plane.get("plane");
  const mesh = actualPlane.get_mesh();
  const name = plane.get("name");
  const upperLeftPos = actualPlane.get_upper_left();
  const upperLeftPosAry = [upperLeftPos.x, upperLeftPos.y, upperLeftPos.z];
  const matrix = actualPlane.get_rotation_matrix();

  const x = new THREE.Vector3(matrix[0][0], matrix[0][1], matrix[0][2]);
  const y = new THREE.Vector3(matrix[1][0], matrix[1][1], matrix[1][2]);
  const z = new THREE.Vector3(matrix[2][0], matrix[2][1], matrix[2][2]);
  const m = new THREE.Matrix4();
  m.makeBasis(x, y, z);
  const a = new THREE.Euler(0, 0, 0, 'XYZ');
  a.setFromRotationMatrix(m, "XYZ");

  const size = 5;
  return <>
    <Solid mesh={mesh} style={"plane"} ></Solid>
    <Wireframe mesh={mesh}></Wireframe>
    <Text
      scale={[size, size, size]}
      color="black" // default
      anchorX="left" // default
      anchorY="top" // default
      depthOffset={-1}
      position={upperLeftPosAry}
      rotation={a}
    >
      {name}
    </Text>
  </>
}

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

export default WorkbenchPane
