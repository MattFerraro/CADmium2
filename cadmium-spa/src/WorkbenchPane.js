import './App.css'
import React, { useEffect, useMemo, useCallback, useRef, useState } from 'react'
import { Canvas } from '@react-three/fiber'
import {
  CameraControls,
  // Environment,
  Text,
  Line,
} from '@react-three/drei'
import * as THREE from 'three'
// import studio_2_1k from './images/studio_2_1k.hdr'
// import { VertexNormalsHelper } from "three/examples/jsm/helpers/VertexNormalsHelper";

// import { useThree } from '@react-three/fiber'

// const sab = new SharedArrayBuffer(1024);
// const ta = new Uint8Array(sab);
let someGlobalValue = 0

function WorkbenchPane({ workbenchView, activeTool, addSegmentToSketch }) {
  let parts = null
  if (workbenchView) {
    parts = workbenchView.solids.map((solid) => solid.get('solid').get_mesh())
  }
  let planes = null
  if (workbenchView) {
    planes = workbenchView.planes
  }
  let sketches = null
  if (workbenchView) {
    sketches = workbenchView.sketches
    console.log('sketches:', sketches)
  }

  const mouseConfig = useCallback((camControls) => {
    if (camControls !== null) {
      camControls.mouseButtons.middle = 16
      camControls.mouseButtons.wheel = 16
      camControls.mouseButtons.right = 1
      camControls.mouseButtons.left = 0
    }
    //  NONE: 0,
    // 	ROTATE: 1,
    // 	TRUCK: 2,
    // 	OFFSET: 4,
    // 	DOLLY: 8,
    //  ZOOM: 16,
    // see https://github.com/yomotsu/camera-controls/blob/29eac5b50e69f0cf6792b8c3c12f5c86ad621222/src/types.ts
  }, [])

  const overallScale = 100

  return (
    <Canvas
      linear={true}
      frameloop="always"
      orthographic
      camera={{
        far: 50000,
        near: 0.0,
        zoom: 4.0,
        position: [1 * overallScale, 0 * overallScale, 0 * overallScale],
        up: [0, 0, 1],
      }}
      style={{
        height: '100%',
        cursor: activeTool === 'line' ? 'crosshair' : 'auto',
      }}
    >
      {/* <Environment files={studio_2_1k} /> */}

      <CameraControls
        ref={mouseConfig}
        dollyToCursor={true}
        maxPolarAngle={900}
      />
      <ambientLight />
      <pointLight
        position={[5 * overallScale, 5 * overallScale, 5 * overallScale]}
      />
      <pointLight
        position={[5 * overallScale, -5 * overallScale, 5 * overallScale]}
      />
      <pointLight
        position={[-5 * overallScale, 5 * overallScale, 5 * overallScale]}
      />
      <pointLight
        position={[-5 * overallScale, -5 * overallScale, 5 * overallScale]}
      />

      <pointLight
        position={[0 * overallScale, 0 * overallScale, -5 * overallScale]}
      />

      {parts &&
        parts.map((part, index) => {
          return <Part key={index} mesh={part}></Part>
        })}

      {planes &&
        planes.map((plane, index) => {
          return <Plane key={index} plane={plane}></Plane>
        })}

      {sketches &&
        sketches.map((sketch, index) => {
          return (
            <Sketch
              key={index}
              sketch={sketch}
              activeTool={activeTool}
              addSegmentToSketch={addSegmentToSketch}
            ></Sketch>
          )
        })}
    </Canvas>
  )
}

function Sketch({ sketch, activeTool, addSegmentToSketch }) {
  const sketchName = useMemo(() => {
    return sketch.get('name')
  }, [sketch])

  // we can use the pointQueue as a...queue!
  const [pointQueue, setPointQueue] = useState([])
  const pushPoint = useCallback(
    (newPoint) => {
      setPointQueue([...pointQueue, newPoint])
    },
    [pointQueue, setPointQueue]
  )
  const popNewestPoint = useCallback(() => {
    const newQueue = pointQueue.slice(0, pointQueue.length - 1)
    setPointQueue(newQueue)
  }, [pointQueue, setPointQueue])
  const popOldestPoint = useCallback(() => {
    const newQueue = pointQueue.slice(1)
    setPointQueue(newQueue)
  }, [pointQueue, setPointQueue])
  const clearQueue = useCallback(() => {
    setPointQueue([])
  }, [setPointQueue])
  const replaceNewestPoint = useCallback(
    (newPoint) => {
      const newQueue = pointQueue.slice(0, pointQueue.length - 1)
      newQueue.push(newPoint)
      setPointQueue(newQueue)
    },
    [pointQueue, setPointQueue]
  )

  useEffect(() => {
    if (activeTool === 'line') {
      if (pointQueue.length > 0) {

        const big_string_array = pointQueue.map((el, idx) => { return "id: " + idx + " el: " + el.x.toFixed(6) + ", " + el.y.toFixed(6) + ", " + el.z.toFixed(6) });
        const big_string = big_string_array.join(", ");

        console.log(
          'Point Queue Changed: ',
          big_string
        )
      }


      let numClicked = 0
      for (let i = 0; i < pointQueue.length; i++) {
        if (pointQueue[i].type === 'clicked') {
          numClicked += 1
        }
      }

      if (numClicked === 2) {
        addSegmentToSketch(
          sketchName,
          pointQueue[0].sketch_x,
          pointQueue[0].sketch_y,
          pointQueue[1].sketch_x,
          pointQueue[1].sketch_y
        )
        popOldestPoint()
      }
    }

  }, [pointQueue, popOldestPoint, activeTool, sketchName, addSegmentToSketch])

  const sketchView = useMemo(() => {
    return sketch.get('sketch')
  }, [sketch])

  const { three_x, three_y, three_z } = useMemo(() => {
    const frame = sketchView.coordinate_frame
    const three_x = new THREE.Vector3(
      frame.x_axis.x,
      frame.x_axis.y,
      frame.x_axis.z
    )
    const three_y = new THREE.Vector3(
      frame.y_axis.x,
      frame.y_axis.y,
      frame.y_axis.z
    )
    const three_z = new THREE.Vector3(
      frame.normal.x,
      frame.normal.y,
      frame.normal.z
    )
    return { three_x, three_y, three_z }
  }, [sketchView])

  const eulerAngles = useMemo(() => {
    const m = new THREE.Matrix4()
    m.makeBasis(three_x, three_y, three_z)
    const ea = new THREE.Euler(0, 0, 0, 'XYZ')
    ea.setFromRotationMatrix(m, 'XYZ')
    return ea
  }, [three_x, three_y, three_z])

  const sketchWidth = 450
  const sketchHeight = 300

  const collisionGeometry = new THREE.PlaneGeometry(20000, 20000)
  const visualGeometry = new THREE.PlaneGeometry(sketchWidth, sketchHeight)
  const edges = new THREE.EdgesGeometry(visualGeometry, 1)
  const textPosition = new THREE.Vector3(-sketchWidth / 2, sketchHeight / 2, 0)
  textPosition.applyEuler(eulerAngles)

  const onClick = (e) => {
    if (pointQueue.length === 0) {
      // this should never happen. If it did IDK what to do.
      return
    }
    const lastPoint = pointQueue[pointQueue.length - 1]
    if (lastPoint.type === 'moved' || lastPoint.type === 'snapped') {
      replaceNewestPoint({
        x: lastPoint.x,
        y: lastPoint.y,
        z: lastPoint.z,
        sketch_x: three_x.dot(lastPoint),
        sketch_y: three_y.dot(lastPoint),
        type: 'clicked',
      })
    }
  }

  const onMouseMove = (e) => {
    if (activeTool === null) {
      return
    }
    if (
      pointQueue.length === 0 ||
      pointQueue[pointQueue.length - 1].type === 'clicked'
    ) {
      pushPoint({
        x: e.point.x,
        y: e.point.y,
        z: e.point.z,
        sketch_x: three_x.dot(e.point),
        sketch_y: three_y.dot(e.point),
        type: 'moved',
      })
    } else {
      if (someGlobalValue === 1) {
        return
      }
      const lastPoint = pointQueue[pointQueue.length - 1]
      if (lastPoint.type === 'moved') {
        replaceNewestPoint({
          x: e.point.x,
          y: e.point.y,
          z: e.point.z,
          sketch_x: three_x.dot(e.point),
          sketch_y: three_y.dot(e.point),
          type: 'moved',
        })
      }
    }
  }

  const onPointerOver = (point, sketch_point) => {
    console.log('on pointer over CB')
    if (
      pointQueue.length > 0 &&
      pointQueue[pointQueue.length - 1].type === 'moved'
    ) {
      console.log('replace with snapped!')
      replaceNewestPoint({
        x: point[0],
        y: point[1],
        z: point[2],
        sketch_x: sketch_point[0],
        sketch_y: sketch_point[1],
        type: 'snapped',
      })
      someGlobalValue = 1
    }
  }

  const onPointerOut = () => {
    console.log('on pointer out CB')
    someGlobalValue = 0
    if (
      pointQueue.length > 0 &&
      pointQueue[pointQueue.length - 1].type === 'snapped'
    ) {
      console.log('Poppped!')
      popNewestPoint()
    }
  }

  useEffect(() => {
    console.log('Clearing queue!')
    clearQueue()
  }, [activeTool, clearQueue])

  // const onPointerOverCb = useCallback((point, sketch_point) => {
  //   console.log("on pointer over", point, sketch_point);

  // here just push the point to a "most recent hovered point" state
  // then consider most recent hovered point in the mouse move and click handlers
  // the goal is NOT to pass the queue, or any callback which depend on the queue,
  // to the SketchPoint component. If we can avoid doing that, then we won't have
  // to rerender the SketchPoint component every time the queue changes.
  // }, []);

  const renderablePoints = useMemo(() => {
    console.log('recomputing renderable Points')
    const thePoints = []
    const uniqueKeys = new Set()
    for (let i = 0; i < sketchView.segments.length; i++) {
      const segment3d = sketchView.segments[i]
      const segment2d = sketchView.segments_2d[i]

      const start_point_3d = segment3d.start
      const end_point_3d = segment3d.end

      const start_point_2d = segment2d.start
      const end_point_2d = segment2d.end

      const start_key_string =
        start_point_3d.x.toFixed(6) +
        ',' +
        start_point_3d.y.toFixed(6) +
        ',' +
        start_point_3d.z.toFixed(6)
      if (!uniqueKeys.has(start_key_string)) {
        uniqueKeys.add(start_key_string)
        thePoints.push({
          '3d': start_point_3d,
          '2d': start_point_2d,
          key: start_key_string,
        })
      }

      const end_key_string =
        end_point_3d.x.toFixed(6) +
        ',' +
        end_point_3d.y.toFixed(6) +
        ',' +
        end_point_3d.z.toFixed(6)
      if (!uniqueKeys.has(end_key_string)) {
        uniqueKeys.add(end_key_string)
        thePoints.push({
          '3d': end_point_3d,
          '2d': end_point_2d,
          key: end_key_string,
        })
      }
    }
    return thePoints
  }, [sketchView])

  const size = 7
  return (
    <>
      {activeTool === 'line' && pointQueue.length >= 2 && (
        <Line
          points={[
            [pointQueue[0].x, pointQueue[0].y, pointQueue[0].z],
            [pointQueue[1].x, pointQueue[1].y, pointQueue[1].z],
          ]}
          color={'#000000'}
          lineWidth={2}
        />
      )}

      <mesh
        rotation={eulerAngles}
        onClick={onClick}
        onPointerMove={onMouseMove}
      >
        <primitive object={collisionGeometry}></primitive>
        <meshStandardMaterial
          color="#FF0000"
          opacity={0.0}
          transparent
          side={THREE.DoubleSide}
          depthWrite={false}
        />
      </mesh>
      <mesh rotation={eulerAngles}>
        <lineSegments
          geometry={edges}
          material={new THREE.LineBasicMaterial({ color: 0x000000 })}
        />
      </mesh>

      <Text
        scale={[size, size, size]}
        color="black" // default
        anchorX="left" // default
        anchorY="top" // default
        depthOffset={0}
        position={textPosition}
        rotation={eulerAngles}
      >
        {sketch.get('name')}
      </Text>
      {sketchView &&
        sketchView.segments.map((segment, index) => {
          return (
            <Line
              key={index}
              points={[
                [segment.start.x, segment.start.y, segment.start.z],
                [segment.end.x, segment.end.y, segment.end.z],
              ]}
              color={'#000000'}
              lineWidth={2}
              depthWrite={false}
            />
          )
        })}

      {sketchView &&
        sketchView.faces_2d.map((face, index) => {
          const face_shape = new THREE.Shape()
          let count = 0
          for (const segment of face.exterior.segments) {
            if (count === 0) {
              face_shape.moveTo(segment.start.x, segment.start.y)
            }
            face_shape.lineTo(segment.end.x, segment.end.y)
            count += 1
          }

          for (const interior of face.interiors) {
            const hole_path = new THREE.Path()

            count = 0
            for (const segment of interior.segments) {
              if (count === 0) {
                hole_path.moveTo(segment.start.x, segment.start.y)
              }
              hole_path.lineTo(segment.end.x, segment.end.y)
              count += 1
            }

            face_shape.holes.push(hole_path)
          }

          const geometry = new THREE.ShapeGeometry(face_shape)
          return (
            <mesh key={index} rotation={eulerAngles}>
              <primitive object={geometry}></primitive>
              <meshStandardMaterial
                color="#006B3C"
                opacity={0.2}
                transparent
                side={THREE.DoubleSide}
                depthWrite={false}
              />
            </mesh>
          )
        })}

      {renderablePoints.map((pointObject, index) => {
        // const segment2d = sketchView.segments_2d[index];
        // console.log("Segment: ", segment, segment2d);
        const point_2d = pointObject['2d']
        const point_3d = pointObject['3d']
        const keyName = pointObject['key']
        return (
          <SketchPoint
            key={keyName}
            x={point_3d.x}
            y={point_3d.y}
            z={point_3d.z}
            sketch_x={point_2d.x}
            sketch_y={point_2d.y}
            onPointerOverCb={onPointerOver}
            onPointerOutCb={onPointerOut}
          ></SketchPoint>
        )
      })}
    </>
  )
}

function Part({ mesh }) {
  return (
    <>
      <Wireframe mesh={mesh} style="solid"></Wireframe>
    </>
  )
}

function Wireframe({ mesh, style }) {
  const ref = useRef()
  const [hovered, hover] = useState(false)
  const positions = new Float32Array(
    mesh.vertices.flatMap((v) => [v.x, v.y, v.z])
  )
  const normals = new Float32Array(mesh.normals.flatMap((v) => [v.x, v.y, v.z]))
  const indices = new Uint16Array(mesh.indices)

  const geometry = new THREE.BufferGeometry()
  geometry.setAttribute('position', new THREE.BufferAttribute(positions, 3))
  geometry.setAttribute('normal', new THREE.BufferAttribute(normals, 3))
  geometry.setIndex(new THREE.BufferAttribute(indices, 1))
  const edges = new THREE.EdgesGeometry(geometry, 1)
  const line = new THREE.LineSegments(
    edges,
    new THREE.LineBasicMaterial({ color: 0x000000 })
  )

  return (
    <>
      <mesh>
        <primitive object={geometry} />
        {style === 'solid' && (
          <meshStandardMaterial
            metalness={0.0}
            roughness={0.0}
            color={hovered ? 'hotpink' : '#006B3C'}
            side={THREE.DoubleSide}
          />
        )}

        {style === 'plane' && (
          <meshStandardMaterial
            color="#606060"
            opacity={0.07}
            transparent
            depthWrite={false}
            side={THREE.DoubleSide}
            polygonOffset={true}
            polygonOffsetFactor={1} // positive value pushes polygon further away
            polygonOffsetUnits={1}
          />
        )}
      </mesh>
      <mesh
        ref={ref}
        onPointerOver={(event) => hover(false)}
        onPointerOut={(event) => hover(false)}
      >
        <lineSegments geometry={edges} material={line.material} />
        <meshStandardMaterial
          // polygonOffset={true}
          // polygonOffsetFactor={1} // positive value pushes polygon further away
          // polygonOffsetUnits={1}
          color={hovered ? 'hotpink' : '#006B3C'}
          side={THREE.DoubleSide}
        />
      </mesh>
    </>
  )
}

function Plane({ plane }) {
  const actualPlane = plane.get('plane')
  const mesh = actualPlane.get_mesh()
  const name = plane.get('name')
  const upperLeftPos = actualPlane.get_upper_left()
  const upperLeftPosAry = [upperLeftPos.x, upperLeftPos.y, upperLeftPos.z]
  const matrix = actualPlane.get_rotation_matrix()

  const x = new THREE.Vector3(matrix[0][0], matrix[0][1], matrix[0][2])
  const y = new THREE.Vector3(matrix[1][0], matrix[1][1], matrix[1][2])
  const z = new THREE.Vector3(matrix[2][0], matrix[2][1], matrix[2][2])
  const m = new THREE.Matrix4()
  m.makeBasis(x, y, z)
  const a = new THREE.Euler(0, 0, 0, 'XYZ')
  a.setFromRotationMatrix(m, 'XYZ')

  const size = 7
  return (
    <>
      <Wireframe mesh={mesh} style={'plane'}></Wireframe>
      <Text
        scale={[size, size, size]}
        color="black" // default
        anchorX="left" // default
        anchorY="top" // default
        depthOffset={0}
        position={upperLeftPosAry}
        rotation={a}
      >
        {name}
      </Text>
    </>
  )
}

const SketchPoint = React.memo(function SketchPoint({
  x,
  y,
  z,
  sketch_x,
  sketch_y,
  onPointerOverCb,
  onPointerOutCb,
}) {
  const point = useMemo(() => [x, y, z], [x, y, z]);
  const sketch_point = useMemo(() => [sketch_x, sketch_y], [sketch_x, sketch_y]);
  const [hovered, hover] = useState(false)

  const dot = useMemo(() => {
    const dotGeometry = new THREE.BufferGeometry()
    dotGeometry.setAttribute(
      'position',
      new THREE.BufferAttribute(new Float32Array([x, y, z]), 3)
    )
    const d = new THREE.Points(dotGeometry)
    return d
  }, [x, y, z])

  const onPointerOver = useCallback(
    (event) => {
      console.log('pointer over: ', point)
      onPointerOverCb(point, sketch_point)
      hover(true)
    },
    [hover, onPointerOverCb, point, sketch_point]
  )

  const onPointerOut = useCallback(
    (event) => {
      onPointerOutCb(point, sketch_point)
      hover(false)
    },
    [hover, onPointerOutCb, point, sketch_point]
  )

  return (
    <mesh
      onPointerOver={onPointerOver}
      onPointerOut={onPointerOut}
    >
      <primitive object={dot}>
        <pointsMaterial
          depthWrite={false}
          color={'#FF0000'}
          size={hovered ? 16 : 7}
        ></pointsMaterial>
      </primitive>
    </mesh>
  )
})

export default WorkbenchPane
