import React, { useState, useEffect, useRef } from "react";
import { gql, useSubscription } from "@apollo/client";

import DeckGL from "@deck.gl/react";
import {
  COORDINATE_SYSTEM,
  OrbitView,
  LinearInterpolator,
} from "@deck.gl/core";
import { PointCloudLayer } from "@deck.gl/layers";

import TopicSelector from "./topic_selector";

const INITIAL_VIEW_STATE = {
  target: [0, 0, 0],
  rotationX: 0,
  rotationOrbit: 0,
  orbitAxis: "Y",
  fov: 50,
  minZoom: 0,
  maxZoom: 10,
  zoom: 1,
};

const transitionInterpolator = new LinearInterpolator(["rotationOrbit"]);

const PointFieldDataTypes = Object.freeze({
  INT8: 1,
  UINT8: 2,
  INT16: 3,
  UINT16: 4,
  INT32: 5,
  UINT32: 6,
  FLOAT32: 7,
  FLOAT64: 8,
});

function getCloudValue(dataView, datatype, endianness, index) {
  switch (datatype) {
    case PointFieldDataTypes.INT8:
      return dataView.getInt8(index, endianness);
    case PointFieldDataTypes.UINT8:
      return dataView.getUint8(index, endianness);
    case PointFieldDataTypes.INT16:
      return dataView.getInt16(index, endianness);
    case PointFieldDataTypes.UINT16:
      return dataView.getUint16(index, endianness);
    case PointFieldDataTypes.INT32:
      return dataView.getInt32(index, endianness);
    case PointFieldDataTypes.UINT32:
      return dataView.getUint32(index, endianness);
    case PointFieldDataTypes.FLOAT32:
      return dataView.getFloat32(index, endianness);
    case PointFieldDataTypes.FLOAT64:
      return dataView.getFloat64(index, endianness);
    default:
      return null;
  }
}

function getNormal(x = 0, y = 0, z = 0) {
  if (x === 0 && y === 0 && z === 0) {
    return {
      normalizedX: 0,
      normalizedY: 0,
      normalizedZ: 0,
    };
  }

  const hypotenuse = Math.sqrt(x * x + y * y + z * z);
  const result = {
    normalizedX: (x / hypotenuse) * 1.5, // 0.5 + 0.5 + 0.5
    normalizedY: (y / hypotenuse) * 1.5, // 0.5 + 0.5 + 0.5
    normalizedZ: (z / hypotenuse) * 1.5, // 0.5 + 0.5 + 0.5
  };

  return result;
}

function getColor(intensity, useRainbow) {
  let r = 0;
  let g = 0;
  let b = 0;

  let min_intensity = 999999.0;
  let max_intensity = -999999.0;

  min_intensity = Math.min(intensity, min_intensity);
  max_intensity = Math.max(intensity, max_intensity);

  min_intensity = Math.max(-999999.0, min_intensity);
  max_intensity = Math.min(999999.0, max_intensity);

  let diff_intensity = max_intensity - min_intensity;
  if (diff_intensity === 0) {
    diff_intensity = 1e20;
  }

  if (useRainbow) {
    let normalized_intensity = (intensity - min_intensity) / diff_intensity;
    normalized_intensity = Math.min(1.0, Math.max(0.0, normalized_intensity));
    r = 255 * normalized_intensity + 0 * (1.0 - normalized_intensity);
    g = 255 * normalized_intensity + 0 * (1.0 - normalized_intensity);
    b = 255 * normalized_intensity + 0 * (1.0 - normalized_intensity);
  } else {
    let value = 1.0 - (intensity - min_intensity) / diff_intensity;
    value = Math.min(value, 1.0);
    value = Math.max(value, 0.0);

    let h = value * 5.0 + 1.0;
    let i = Math.floor(h);
    let f = h - i;
    if (!(i & 1)) {
      f = 1.0 - f;
    }
    let n = 1.0 - f;

    if (i <= 1) {
      r = n * 255.0;
      g = 0.0;
      b = 255.0;
    } else if (i === 2) {
      r = 0.0;
      g = n * 255.0;
      b = 255.0;
    } else if (i === 3) {
      r = 0.0;
      g = 255.0;
      b = n * 255.0;
    } else if (i === 4) {
      r = n * 255.0;
      g = 255.0;
      b = 0.0;
    } else if (i >= 5) {
      r = 255.0;
      g = n * 255.0;
      b = 0.0;
    }
  }

  return {
    r: r,
    g: g,
    b: b,
  };
}

function parsePointCloud(binaryData) {
  if (binaryData.byteLength < 8) {
    return {
      position: [],
      normal: [],
      color: [],
    };
  }

  let headerDataSize = new DataView(binaryData, 0, 8).getBigUint64(0, true);
  let headerDataView = new DataView(binaryData, 8, Number(headerDataSize));
  let binaryDataView = new DataView(binaryData, 8 + Number(headerDataSize));
  let headerData = JSON.parse(new TextDecoder().decode(headerDataView));

  const result = [];
  const LITTLE_ENDIAN = !headerData.is_bigendian;
  const pointField_x = headerData.fields[0];
  const pointField_y = headerData.fields[1];
  const pointField_z = headerData.fields[2];
  const pointField_intensity = headerData.fields[3];

  const useRainbow = false;

  const size = binaryData.byteLength - (8 + Number(headerDataSize));

  for (let row = 0; row < size; row += headerData.point_step) {
    const x = getCloudValue(
      binaryDataView,
      pointField_x.datatype,
      LITTLE_ENDIAN,
      row + pointField_x.offset
    );
    const y = getCloudValue(
      binaryDataView,
      pointField_y.datatype,
      LITTLE_ENDIAN,
      row + pointField_y.offset
    );
    const z = getCloudValue(
      binaryDataView,
      pointField_z.datatype,
      LITTLE_ENDIAN,
      row + pointField_z.offset
    );
    const intensity = getCloudValue(
      binaryDataView,
      pointField_intensity.datatype,
      LITTLE_ENDIAN,
      row + pointField_intensity.offset
    );

    const normals = getNormal(x, y, z);
    const color = getColor(intensity, useRainbow);

    result.push({
      position: [x, y, z],
      normal: [normals.normalizedX, normals.normalizedY, normals.normalizedZ],
      color: [color.r, color.g, color.b],
    });
  }

  return result;
}

const DeckGLStyle = {
  position: "relative",
};

const StreamViewer = ({ onLoad }) => {
  const ws = useRef(null);
  const [viewState, updateViewState] = useState(INITIAL_VIEW_STATE);
  const [isLoaded, setIsLoaded] = useState(false);
  const [binaryData, setBinaryData] = useState();
  const [pointCloudData, setPointCloudData] = useState();
  const [topic, setTopic] = useState(null);

  useEffect(() => {
    if (!topic) {
      return;
    }

    ws.current = new WebSocket("ws://192.168.1.124:8000/ws");
    ws.current.binaryType = "arraybuffer";
    ws.current.onopen = () => {
      ws.current.send(
        JSON.stringify({
          request: "STREAM_READ",
          body: {
            kind: "POINTCLOUD2",
            topic: topic.value,
            msg_type: topic.datatype,
          },
        })
      );
      console.log("ws opened");
    };
    ws.current.onclose = () => console.log("ws closed");
    ws.current.onerror = (error) => {
      console.log(`[error] ${error.message}`);
    };

    ws.current.onmessage = (e) => {
      if (e.data instanceof ArrayBuffer) {
        setBinaryData(e.data);
      } else if (e.data instanceof Blob) {
      }
    };

    return () => {
      ws.current.close();
    };
  }, [topic]);

  useEffect(() => {
    if (!isLoaded) {
      return;
    }

    const rotateCamera = () => {
      updateViewState((v) => ({
        ...v,
        rotationOrbit: v.rotationOrbit + 120,
        transitionDuration: 2400,
        transitionInterpolator,
        onTransitionEnd: rotateCamera,
      }));
    };
    rotateCamera();
  }, [isLoaded]);

  useEffect(() => {
    if (binaryData === undefined) {
      return;
    }

    const data = parsePointCloud(binaryData);
    setPointCloudData(data);
  }, [binaryData]);

  const onSelectedChanged = (selectedOption) => {
    if (selectedOption) {
      setTopic(selectedOption);
    }
  };

  const onDataLoad = ({ header }) => {
    console.log(header);
    if (header.boundingBox) {
      const [mins, maxs] = header.boundingBox;
      // File contains bounding box info
      updateViewState({
        ...INITIAL_VIEW_STATE,
        target: [
          (mins[0] + maxs[0]) / 2,
          (mins[1] + maxs[1]) / 2,
          (mins[2] + maxs[2]) / 2,
        ],
        // global window
        zoom: Math.log2(window.innerWidth / (maxs[0] - mins[0])) - 1,
      });
      setIsLoaded(true);
    }

    if (onLoad) {
      onLoad({ count: header.vertexCount, progress: 1 });
    }
  };

  const layers = [
    new PointCloudLayer({
      id: "stream-layer",
      data: pointCloudData,
      onDataLoad,
      coordinateSystem: COORDINATE_SYSTEM.CARTESIAN,
      getPosition: (d) => d.position,
      getNormal: (d) => d.normal,
      getColor: (d) => d.color,
      opacity: 0.8,
      pointSize: 1,
    }),
  ];

  return (
    <DeckGL
      style={DeckGLStyle}
      views={new OrbitView()}
      viewState={viewState}
      controller={true}
      onViewStateChange={(v) => updateViewState(v.viewState)}
      layers={layers}
      parameters={{
        clearColor: [0.1, 0.1, 0.1, 1],
      }}
    >
      <TopicSelector onSelectedChanged={onSelectedChanged} />
    </DeckGL>
  );
};

export default StreamViewer;
