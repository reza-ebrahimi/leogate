import React, { useState, useEffect } from "react";
import styled from "styled-components";
import Select from "react-select";
import { gql, useQuery } from "@apollo/client";
import Network from "../network_interface";

const TOPICS = gql`
  query {
    masterInfo {
      topics {
        name
        datatypes
      }
    }
  }
`;

const customStyles = {
  option: (provided, state) => ({
    ...provided,
    color: "black",
  }),
};

const TopicSelectorStyled = styled.div`
  width: 400px;
  margin-top: 10px;
  margin-left: 10px;
`;

var options = [];

const TopicSelector = ({ onSelectedChanged }) => {
  const [selectedOption, setSelectedOption] = useState(null);
  const { loading, error, data } = useQuery(TOPICS, {
    client: Network.default_client.handle,
  });

  useEffect(() => {
    if (loading || error) {
      return;
    }

    data.masterInfo.topics.map((topic, idx) => {
      topic.datatypes.map((datatype) => {
        let name = topic.name.toLowerCase();
        if (datatype === "sensor_msgs/PointCloud2") {
          options.push({
            value: name,
            label: name,
            datatype: datatype,
          });
        }
        return datatype;
      });
      return topic;
    });

    return () => {
      options = [];
    };
  }, [loading, error, data]);

  useEffect(() => {
    onSelectedChanged(selectedOption);
  }, [selectedOption, onSelectedChanged]);

  if (error) return `Error! ${error.message}`;
  if (loading) {
    return null;
  }

  return (
    <TopicSelectorStyled>
      <Select
        placeholder="Select a topic"
        styles={customStyles}
        defaultValue={selectedOption}
        onChange={setSelectedOption}
        options={options}
      />
    </TopicSelectorStyled>
  );
};

export default TopicSelector;
