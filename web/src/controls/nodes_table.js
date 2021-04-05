import React from "react";
import { makeStyles } from "@material-ui/core/styles";
import Table from "@material-ui/core/Table";
import TableBody from "@material-ui/core/TableBody";
import TableCell from "@material-ui/core/TableCell";
import TableContainer from "@material-ui/core/TableContainer";
import TableHead from "@material-ui/core/TableHead";
import TableRow from "@material-ui/core/TableRow";
import Paper from "@material-ui/core/Paper";

const useStyles = makeStyles({
  containerStyle: {
    background: "transparent",
    border: "3px solid gray",
    borderRadius: 0,
    //maxHeight: 300,
  },
  tableStyle: {
    minWidth: 650,
  },
  cellStyle: {
    color: "white",
  },
});

const NodesInfo = ({ nodes }) => {
  const classes = useStyles();

  return (
    <TableContainer className={classes.containerStyle} component={Paper}>
      <Table aria-label="nodes table">
        <TableHead>
          <TableRow>
            <TableCell className={classes.cellStyle}>Node Name</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {nodes.map((node, idx) => (
            <TableRow key={idx}>
              <TableCell className={classes.cellStyle}>{node}</TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </TableContainer>
  );
};

export default NodesInfo;
