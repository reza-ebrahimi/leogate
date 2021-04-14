import React, { useEffect } from "react";
import styled from "styled-components";
import { gql, useQuery } from "@apollo/client";

const QUERY = gql`
  query {
    osInfo {
      osType
      version
      edition
      codename
    }
    serverInfo {
      cpuTemp
      bootTime
      uptime
      onAcPower
    }
    cpuLoad {
      user
      nice
      system
      interrupt
      idle
    }
    networks {
      interfaces
    }
  }
`;

const QUERY_POLLL_INTERVAL_MS = 500;

const ServerInfo = ({ data }) => {
  const info = data.serverInfo;

  return (
    <SystemStatusContainer header="Server Information">
      <div>CPU Temperature:</div>
      <div>{info.cpuTemp}</div>
      <div>Boot Time:</div>
      <div>{info.bootTime}</div>
      <div>Up Time:</div>
      <div>{info.uptime}</div>
      <div>On AC Power:</div>
      <div>{info.onAcPower ? "Yes" : "No"}</div>
    </SystemStatusContainer>
  );
};

const OsInfo = ({ data }) => {
  const info = data.osInfo;

  return (
    <SystemStatusContainer header="OS Information">
      <div>OS Type:</div>
      <div>{info.osType}</div>
      <div>OS Version:</div>
      <div>{info.version}</div>
      {info.edition && (
        <>
          <div>OS Edition:</div>
          <div>{info.edition}</div>
        </>
      )}
      <div>Code name:</div>
      <div>{info.codename}</div>
    </SystemStatusContainer>
  );
};

const CpuLoad = ({ data }) => {
  const stat = data.cpuLoad;

  return (
    <SystemStatusContainer header="CPU Load (%)">
      <div>User:</div>
      <div>{stat.user.toFixed(2)}%</div>
      <div>Nice:</div>
      <div>{stat.nice.toFixed(2)}%</div>
      <div>System:</div>
      <div>{stat.system.toFixed(2)}%</div>
      <div>Interrupt:</div>
      <div>{stat.interrupt.toFixed(2)}%</div>
      <div>Idle:</div>
      <div>{stat.idle.toFixed(2)}%</div>
    </SystemStatusContainer>
  );
};

const Networks = ({ data }) => {
  const networks = data.networks;

  return (
    <SystemStatusContainer header="Networks [Name, IP Address]">
      {Object.entries(networks.interfaces).map(([network, addr], idx) => {
        return (
          <React.Fragment key={idx}>
            <div>{network}</div>
            {addr.length > 0 ? <div>{addr[0]}</div> : <div>N/A</div>}
          </React.Fragment>
        );
      })}
    </SystemStatusContainer>
  );
};

const SystemStatusContainerStyled = styled.div`
  display: grid;
  grid-template-columns: 1fr 1fr;
  grid-row-gap: 10px;
  align-self: start;
  padding-top: 15px;
  margin-top: 5px;
  border-top: 1px solid silver;
`;

const SystemStatusPanelStyled = styled.div`
  display: grid;
  grid-template-columns: 1fr 1fr 1fr 1fr;
  grid-column-gap: 20px;
`;

const SystemStatusContainer = (props) => {
  return (
    <div>
      {props.header && <div>{props.header}</div>}
      <SystemStatusContainerStyled>
        {props.children}
      </SystemStatusContainerStyled>
    </div>
  );
};

const SystemStatusPanel = (props) => {
  const { loading, error, data, startPolling, stopPolling } = useQuery(QUERY);

  useEffect(() => {
    startPolling(QUERY_POLLL_INTERVAL_MS);
    return () => {
      stopPolling();
    };
  }, [startPolling, stopPolling]);

  if (error) {
    return `Query Error! ${error.message}`;
  }

  if (loading) {
    return null;
  }

  return (
    <SystemStatusPanelStyled>
      <OsInfo data={data} />
      <ServerInfo data={data} />
      <Networks data={data} />
      <CpuLoad data={data} />
    </SystemStatusPanelStyled>
  );
};

export default SystemStatusPanel;
