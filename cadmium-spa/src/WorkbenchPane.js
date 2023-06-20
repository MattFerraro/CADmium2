import './App.css'
import React, { useEffect, useMemo, useCallback, useRef, useState } from 'react'
import { Canvas, useFrame } from '@react-three/fiber'
import { CameraControls, Environment, useHelper, Text, Line } from '@react-three/drei'
import * as THREE from 'three'
// import studio_2_1k from './images/studio_2_1k.hdr'
// import { VertexNormalsHelper } from "three/examples/jsm/helpers/VertexNormalsHelper";

// import { useThree } from '@react-three/fiber'

function WorkbenchPane({ workbenchView, activeTool, addSegmentToSketch }) {

  let parts = null;
  if (workbenchView) {
    parts = workbenchView.solids.map((solid) => solid.get("solid").get_mesh());
  }
  let planes = null;
  if (workbenchView) {
    planes = workbenchView.planes;
  }
  let sketches = null;
  if (workbenchView) {
    sketches = workbenchView.sketches;
    console.log("sketches:", sketches);
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

  const overallScale = 100;

  return (
    <Canvas linear={true} frameloop='always' orthographic camera={{ far: 50000, near: 0.0, zoom: 4.0, position: [1 * overallScale, 0 * overallScale, 0 * overallScale], up: [0, 0, 1] }} style={{ height: '100%', cursor: activeTool === "line" ? "crosshair" : "auto" }}>
      {/* <Environment files={studio_2_1k} /> */}

      <CameraControls ref={mouseConfig} dollyToCursor={true} maxPolarAngle={900} />
      <ambientLight />
      <pointLight position={[5 * overallScale, 5 * overallScale, 5 * overallScale]} />
      <pointLight position={[5 * overallScale, -5 * overallScale, 5 * overallScale]} />
      <pointLight position={[-5 * overallScale, 5 * overallScale, 5 * overallScale]} />
      <pointLight position={[-5 * overallScale, -5 * overallScale, 5 * overallScale]} />

      <pointLight position={[0 * overallScale, 0 * overallScale, -5 * overallScale]} />

      {parts && parts.map((part, index) => {
        return <Part key={index} mesh={part}></Part>
      })}

      {planes && planes.map((plane, index) => {
        return <Plane key={index} plane={plane}></Plane>
      })}

      {sketches && sketches.map((sketch, index) => {
        return <Sketch
          key={index}
          sketch={sketch}
          activeTool={activeTool}
          addSegmentToSketch={addSegmentToSketch}>
        </Sketch>
      })}
    </Canvas>
  )
}

function Sketch({ sketch, activeTool, addSegmentToSketch }) {
  console.log("RENDER SKETCH with faces: ", sketch.get("sketch").faces.length, " and segments: ", sketch.get("sketch").segments.length);

  const [anchorPoint, setAnchorPoint] = useState(null);
  const [secondPoint, setSecondPoint] = useState(null);
  const [heldPoint, setHeldPoint] = useState(null);
  const [cursorSnapped, setCursorSnapped] = useState(false);
  const sketchView = useMemo(() => {
    return sketch.get("sketch")
  }, [sketch]);

  const { three_x, three_y, three_z } = useMemo(() => {
    const frame = sketchView.coordinate_frame;
    const three_x = new THREE.Vector3(frame.x_axis.x, frame.x_axis.y, frame.x_axis.z);
    const three_y = new THREE.Vector3(frame.y_axis.x, frame.y_axis.y, frame.y_axis.z);
    const three_z = new THREE.Vector3(frame.normal.x, frame.normal.y, frame.normal.z);
    return { three_x, three_y, three_z }
  }, [sketchView]);

  const eulerAngles = useMemo(() => {
    const m = new THREE.Matrix4();
    m.makeBasis(three_x, three_y, three_z);
    const ea = new THREE.Euler(0, 0, 0, 'XYZ');
    ea.setFromRotationMatrix(m, "XYZ");
    return ea
  }, [three_x, three_y, three_z]);

  const sketchWidth = 450;
  const sketchHeight = 300;

  const collisionGeometry = new THREE.PlaneGeometry(20000, 20000);
  const visualGeometry = new THREE.PlaneGeometry(sketchWidth, sketchHeight);
  const edges = new THREE.EdgesGeometry(visualGeometry, 1);
  const textPosition = new THREE.Vector3(-sketchWidth / 2, sketchHeight / 2, 0);
  textPosition.applyEuler(eulerAngles);

  const onClick = (e) => {
    if (activeTool === "line") {
      if (anchorPoint === null) {
        setAnchorPoint(e.point);
      } else {
        const x1 = three_x.dot(anchorPoint);
        const y1 = three_y.dot(anchorPoint);

        let x2, y2;
        if (cursorSnapped) {
          // x2 = secondPoint.x;
          // y2 = secondPoint.y;
          x2 = heldPoint.x;
          y2 = heldPoint.y;
        } else {
          x2 = three_x.dot(e.point);
          y2 = three_y.dot(e.point);
        }

        addSegmentToSketch(sketch.get("name"), x1, y1, x2, y2);
        setAnchorPoint(new THREE.Vector3(secondPoint.x, secondPoint.y, secondPoint.z));
        setSecondPoint(null);
        setCursorSnapped(false);
      }
    }
  }

  const onMouseMove = (e) => {
    if (activeTool === "line") {
      if (anchorPoint !== null) {
        if (!cursorSnapped) {
          setSecondPoint(e.point);
        }
      }
    }
  }

  const setSecondaryCallback = useCallback((point, sketch_point) => {
    console.log("Setting secondary!", point, sketch_point);
    setCursorSnapped(true);
    setHeldPoint(new THREE.Vector3(sketch_point[0], sketch_point[1], 0));
    setSecondPoint(new THREE.Vector3(point[0], point[1], point[2]));
  }, []);

  const releaseSecondaryCallback = useCallback(() => {
    console.log("Releasing secondary!");
    setCursorSnapped(false);
  }, []);

  useEffect(() => {
    if (activeTool === null) {
      setAnchorPoint(null);
      setSecondPoint(null);
      setHeldPoint(null);
    }
  }, [activeTool])

  const size = 7;
  return <>
    {anchorPoint && secondPoint && activeTool === "line" &&
      <Line
        points={[
          [anchorPoint.x, anchorPoint.y, anchorPoint.z],
          [secondPoint.x, secondPoint.y, secondPoint.z],
        ]}
        color={"#000000"}
        lineWidth={2}
      />
    }

    <mesh rotation={eulerAngles} onClick={onClick} onPointerMove={onMouseMove}>
      <primitive object={collisionGeometry}></primitive>
      <meshStandardMaterial
        color="#FF0000" opacity={0.0} transparent
        side={THREE.DoubleSide}
        depthWrite={false}
      />
    </mesh>
    <mesh rotation={eulerAngles}>
      <lineSegments geometry={edges} material={new THREE.LineBasicMaterial({ color: 0x000000 })} />
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
      {sketch.get("name")}
    </Text>
    {sketchView && sketchView.segments.map((segment, index) => {
      return <Line
        key={index}
        points={[
          [segment.start.x, segment.start.y, segment.start.z],
          [segment.end.x, segment.end.y, segment.end.z],
        ]}
        color={"#000000"}
        lineWidth={2}
        depthWrite={false}
      />
    })}

    {sketchView && sketchView.faces_2d.map((face, index) => {
      const face_shape = new THREE.Shape();
      let count = 0;
      for (const segment of face.exterior.segments) {
        if (count == 0) {
          face_shape.moveTo(segment.start.x, segment.start.y);
        }
        face_shape.lineTo(segment.end.x, segment.end.y);
        count += 1;
      }

      for (const interior of face.interiors) {
        const hole_path = new THREE.Path();

        count = 0;
        for (const segment of interior.segments) {
          if (count == 0) {
            hole_path.moveTo(segment.start.x, segment.start.y);
          }
          hole_path.lineTo(segment.end.x, segment.end.y);
          count += 1;
        }

        face_shape.holes.push(hole_path);

      }

      const geometry = new THREE.ShapeGeometry(face_shape);
      return <mesh key={index} rotation={eulerAngles}>
        <primitive object={geometry}></primitive>
        <meshStandardMaterial
          color="#006B3C" opacity={0.2} transparent
          side={THREE.DoubleSide}
          depthWrite={false}
        />
      </mesh>
    })}

    {sketchView && sketchView.segments.map((segment, index) => {
      const segment2d = sketchView.segments_2d[index];
      // console.log("Segment: ", segment, segment2d);
      return <SketchPoint
        key={index}
        x={segment.start.x} y={segment.start.y} z={segment.start.z}
        sketch_x={segment2d.start.x} sketch_y={segment2d.start.y}
        setSecondaryCallback={setSecondaryCallback}
        releaseSecondaryCallback={releaseSecondaryCallback}
      ></SketchPoint >
    })}
  </>

}

function Part({ mesh }) {
  return <>
    <Wireframe mesh={mesh} style="solid"></Wireframe>
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
        color={hovered ? 'hotpink' : '#006B3C'}
        side={THREE.DoubleSide}
      />}

      {style === "plane" && <meshStandardMaterial
        color="#006B3C" opacity={0.1} transparent
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

function Wireframe({ mesh, style }) {
  const ref = useRef()
  const [hovered, hover] = useState(false)
  const positions = new Float32Array(mesh.vertices.flatMap((v) => [v.x, v.y, v.z]));
  const normals = new Float32Array(mesh.normals.flatMap((v) => [v.x, v.y, v.z]));
  const indices = new Uint16Array(mesh.indices);

  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute('position', new THREE.BufferAttribute(positions, 3));
  geometry.setAttribute('normal', new THREE.BufferAttribute(normals, 3));
  geometry.setIndex(new THREE.BufferAttribute(indices, 1));
  const edges = new THREE.EdgesGeometry(geometry, 1);
  const line = new THREE.LineSegments(edges, new THREE.LineBasicMaterial({ color: 0x000000 }));

  return (
    <>
      <mesh>
        <primitive object={geometry} />
        {style === "solid" && <meshStandardMaterial
          metalness={0.0}
          roughness={0.0}
          color={hovered ? 'hotpink' : '#006B3C'}
          side={THREE.DoubleSide}
        />}

        {style === "plane" && <meshStandardMaterial
          color="#606060"
          opacity={0.07}
          transparent
          depthWrite={false}
          side={THREE.DoubleSide}
          polygonOffset={true}
          polygonOffsetFactor={1} // positive value pushes polygon further away
          polygonOffsetUnits={1}
        />}
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

  const size = 7;
  return <>
    <Wireframe mesh={mesh} style={"plane"}></Wireframe>
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

const SketchPoint = React.memo(function SketchPoint({ activeTool, x, y, z, sketch_x, sketch_y, setSecondaryCallback, releaseSecondaryCallback }) {
  const point = [x, y, z];
  const sketch_point = [sketch_x, sketch_y];
  const [hovered, hover] = useState(false)
  const dotGeometry = new THREE.BufferGeometry();
  dotGeometry.setAttribute('position', new THREE.BufferAttribute(new Float32Array(point), 3));
  const dotMaterial = new THREE.PointsMaterial({
    size: hovered ? 16 : 7,
    color: 0xff0000,
    depthWrite: false,
  });
  const dot = new THREE.Points(dotGeometry, dotMaterial);

  const onPointerOver = useCallback((event) => {
    // console.log("pointer over: ", point);
    setSecondaryCallback(point, sketch_point);
    hover(true)
  }, [x, y, z, sketch_x, sketch_y])

  const onPointerOut = useCallback((event) => {
    // console.log("off: ", point);
    releaseSecondaryCallback();
    hover(false)
  }, [])

  // const onPointerClick = useCallback((event) => {
  //   console.log("CLICKED ON: ", point);
  // }, [x, y, z]);

  return <mesh
    // onPointerClick={onPointerClick}
    onPointerOver={onPointerOver}
    onPointerOut={onPointerOut}>
    <primitive object={dot}></primitive>
  </mesh>
});

export default WorkbenchPane
