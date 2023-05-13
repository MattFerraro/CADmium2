import './App.css'
import React, { useCallback, useEffect, useRef, useState } from 'react'
import { Canvas, useFrame } from '@react-three/fiber'
import { CameraControls } from '@react-three/drei'
import * as THREE from 'three'
// import init, * as Truck from 'truck-js'
// import { initSync } from 'cadmium-js'
import { default as init, greet } from "cadmium-js";
// const CAD = import('cadmium-js');

import { useThree } from '@react-three/fiber'

function MainViewport() {
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

  const overallScale = 1.1;

  return (
    <Canvas camera={{ fov: 45, position: [1 * overallScale, 2 * overallScale, 1 * overallScale], up: [0, 0, 1] }} style={{ height: 600 }}>
      <TruckHandler></TruckHandler>
      <CameraControls ref={mouseConfig} />
      <ambientLight />
      <pointLight position={[10, 10, 10]} />
      {/* <Plane></Plane> */}
      <axesHelper></axesHelper>
    </Canvas>
  )
}



function TruckHandler(props) {
  const state = useThree()
  useEffect(() => {
    console.log("Hello!");

    console.log(init)
    init().then(() => {
      greet("matt");
    });

    // initSync().then(() => {
    //   console.log("loaded?");
    // })
    // console.log(CAD.greet("hi"));
    // CAD.initSync();
    // CAD.init();
    // init().then(() => {
    //   console.log("initialized!");
    // })

    // init().then(() => {
    //   const scene = state.scene

    //   const v = Truck.vertex(-0.5, -0.5, -0.5)
    //   const e = Truck.tsweep(v.upcast(), [1.0, 0.0, 0.0])
    //   const f = Truck.tsweep(e, [0.0, 1.0, 0.0])
    //   const abst = Truck.tsweep(f, [0.0, 0.0, 1.0])
    //   const solid = abst.into_solid()

    //   const v2 = Truck.vertex(-0.25, -0.25, -0.75)
    //   const e2 = Truck.tsweep(v2.upcast(), [.5, 0.0, 0.0])
    //   const f2 = Truck.tsweep(e2, [0.0, .5, 0.0])
    //   const abst2 = Truck.tsweep(f2, [0.0, 0.0, 1.5])
    //   const solid2 = abst2.into_solid()

    //   const solid3 = Truck.and(solid, Truck.not(solid2))

    //   let polygon = solid3.to_polygon(0.01)
    //   const object = polygon.to_buffer()
    //   let vBuffer = Array.from(object.vertex_buffer())
    //   // vBuffer looks like:
    //   // [x, y, z, u, v, nx, ny, nz]
    //   let vertices = []
    //   let uvs = []
    //   let normals = []
    //   for (let idx = 0; idx < vBuffer.length; idx++) {
    //     const mod = idx % 8
    //     const val = vBuffer[idx]
    //     if (mod === 0 || (mod === 1) | (mod === 2)) {
    //       vertices.push(val)
    //     } else if (mod === 3 || mod === 4) {
    //       uvs.push(val)
    //     } else {
    //       normals.push(val)
    //     }
    //   }
    //   // console.log('vertices', vertices)

    //   let iBuffer = object.index_buffer()
    //   iBuffer = Array.from(iBuffer)
    //   let indexLength = object.index_buffer_size() / 4

    //   const geometry = new THREE.BufferGeometry()

    //   // console.log('vBuffer', vBuffer)
    //   // console.log('iBuffer', iBuffer)
    //   // console.log('length', indexLength)

    //   geometry.setAttribute(
    //     'position',
    //     new THREE.Float32BufferAttribute(vertices, 3)
    //   )
    //   geometry.setAttribute(
    //     'normal',
    //     new THREE.Float32BufferAttribute(normals, 3)
    //   )
    //   geometry.setAttribute(
    //     'uv',
    //     new THREE.Float32BufferAttribute(uvs, 3)
    //   )
    //   geometry.setIndex(iBuffer)

    //   const material = new THREE.MeshNormalMaterial({
    //     color: 0xff0000,
    //     side: THREE.DoubleSide,
    //   })
    //   const mesh = new THREE.Mesh(geometry, material)
    //   scene.add(mesh)

    //   // geometry.setAttribute('normal', new THREE.Float32BufferAttribute(normals, 3));
    //   // geometry.setAttribute('color', new THREE.Float32BufferAttribute(colors, 3));

    //   // greet("WebAssembly")
    //   //   const matt = demo();
    //   //   const jsony = JSON.parse(new TextDecoder().decode(matt))
    //   //    console.log("um, hi", jsony);
    //   // const vAttributes = createVbo(gl, vBuffer);
    //   // gl.bindBuffer(gl.ARRAY_BUFFER, vAttributes);

    //   // const vPositionLocation = gl.getAttribLocation(prg, "position");
    //   // const vUVLocation = gl.getAttribLocation(prg, "uv");
    //   // const vNormalLocation = gl.getAttribLocation(prg, "normal");

    //   // const vIndex = createIbo(ctx, iBuffer);
    //   // console.log(vIndex)
    // })
  }, [])
  return <></>
}

function Plane(props) {
  const ref = useRef()
  const [hovered, hover] = useState(false)
  return (
    <mesh
      {...props}
      ref={ref}
      onPointerOver={(event) => hover(true)}
      onPointerOut={(event) => hover(false)}
    >
      <planeGeometry args={[2, 2]} />
      <meshStandardMaterial
        color={hovered ? 'hotpink' : 'orange'}
        side={THREE.DoubleSide}
      />
    </mesh>
  )
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

export default MainViewport
