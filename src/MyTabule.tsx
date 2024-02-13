import * as React from 'react';
import { alpha } from '@mui/material/styles';
import Box from '@mui/material/Box';
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TablePagination from '@mui/material/TablePagination';
import TableRow from '@mui/material/TableRow';
import TableSortLabel from '@mui/material/TableSortLabel';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import Paper from '@mui/material/Paper';
import Checkbox from '@mui/material/Checkbox';
import IconButton from '@mui/material/IconButton';
import Tooltip from '@mui/material/Tooltip';
import DeleteIcon from '@mui/icons-material/Delete';
import FilterListIcon from '@mui/icons-material/FilterList';
import { visuallyHidden } from '@mui/utils';
import { useState } from 'react';

type Order = 'asc' | 'desc';




export interface Column {
  id: string;
  label: string;
  minWidth?: number;
  align?: 'left' | 'right' | 'center';
  format?: ((value: number) => string) |
  ((value: string[]) => string) |
  ((value: boolean) => string);
}


interface EnhancedTableProps {
  numSelected: number;
  onRequestSort: (property: string) => void;
  onSelectAllClick: (event: React.ChangeEvent<HTMLInputElement>) => void;
  order: Order;
  orderBy: string;
  rowCount: number;
  headCells: readonly Column[];
}

function EnhancedTableHead(props: EnhancedTableProps) {
  const { numSelected,
    onRequestSort,
    onSelectAllClick,
    order,
    orderBy,
    rowCount,
    headCells } = props;
  const createSortHandler =
    (property: string) => () => {
      onRequestSort(property);
    };

    onSelectAllClick

  return (
    <TableHead>
      <TableRow>
        <TableCell padding="checkbox">
          <Checkbox
            color="primary"
            indeterminate={numSelected > 0 && numSelected < rowCount}
            checked={rowCount > 0 && numSelected === rowCount}
            onChange={onSelectAllClick}
            inputProps={{
              'aria-label': 'select all desserts',
            }}
          />
        </TableCell>
        {props.headCells.map((headCell) => (
          <TableCell

            key={headCell.id}
            align={headCell.align}
            padding={'normal'}
            sortDirection={orderBy === headCell.id ? order : false}
          >
            <TableSortLabel
              active={orderBy === headCell.id}
              direction={orderBy === headCell.id ? order : 'asc'}
              onClick={createSortHandler(headCell.id)}
            >
              {headCell.label}
              {orderBy === headCell.id ? (
                <Box component="span" sx={visuallyHidden}>
                  {order === 'desc' ? 'sorted descending' : 'sorted ascending'}
                </Box>
              ) : null}
            </TableSortLabel>
          </TableCell>
        ))}
      </TableRow>
    </TableHead>
  );
}

interface EnhancedTableToolbarProps {
  numSelected: number;
  name: string;
  checked: boolean;
  onChange: ( p_any:any) => any ;
}

function EnhancedTableToolbar(props: EnhancedTableToolbarProps) {
  const { numSelected, name, checked, onChange } = props;

  return (
    <Toolbar
      sx={{
        pl: { sm: 2 },
        pr: { xs: 1, sm: 1 },
        ...(numSelected > 0 && {
          bgcolor: (theme) =>
            alpha(theme.palette.primary.main, theme.palette.action.activatedOpacity),
        }),
      }}
    >

      {numSelected > 0 ? (
        <Typography
          sx={{ flex: '1 1 100%' }}
          color="inherit"
          variant="subtitle1"
          component="div"
        >
          {numSelected} selected
        </Typography>
      ) : (
        <Typography
          sx={{ flex: '1 1 100%' }}
          variant="h6"
          id="tableTitle"
          component="div"
        >
          {name}
        </Typography>
      )}
      {numSelected > 0 ? (
        <Tooltip title="Delete">
          <IconButton>
            <DeleteIcon />
          </IconButton>
        </Tooltip>
      ) : (
        <Tooltip title="Filter list">
          <IconButton>
            <FilterListIcon />
          </IconButton>
        </Tooltip>
      )}

    </Toolbar>
  );
}


interface MyTableProps<T extends Record<any, any>> {
  data: T[];
  columns: readonly Column[];
  name: string;
}



export function MyTable<T extends Record<string | number, any>>({ data, columns, name }: MyTableProps<T>) {

  if (data.length == 0) {
    return <p>data is empty</p>
  } else {

    const [order, setOrder] = React.useState<Order>('asc');
    const [orderBy, setOrderBy] = React.useState<string>(Object.keys(data[0] as Record<string, any>)[0]);
    const [selected, setSelected] = React.useState<number[]>([]);
    const [page, setPage] = React.useState(0);
    const [dense, setDense] = React.useState(true);
    const [rowsPerPage, setRowsPerPage] = React.useState(5);


    React.useEffect(() => {
      data.sort((a, b) => (order === 'asc' ? 1 : -1) * (a[orderBy as keyof T] > b[orderBy as keyof T] ? 1 : -1))
    }, [orderBy, order])


    const handleRequestSort = (
      property: string,
    ) => {
      const isAsc = orderBy === property && order === 'asc';
      setOrder(isAsc ? 'desc' : 'asc');
      setOrderBy(property);
    };

    //  const handleSelectAllClick = (event: React.ChangeEvent<HTMLInputElement>) => {
    //    if (event.target.checked) {
    //      const newSelected = data.map((n) =>  n[id  as keyof T] );
    //      setSelected(newSelected  );
    //      return;
    //    }
    //    setSelected([]);
    //  };
    const [updated , setupdated] = useState<number>(1); 
     React.useEffect(()=>{
       console.log("useEffect" , selected)

     } ,[ selected,updated  ,  updated] )   
    const handleClick = (id:number) => {
      console.log("befor" , id)
      console.log("befor" , selected)
      if (selected.indexOf(id) >= 0) {
        selected.splice(selected.indexOf(id) , 1 ); 
        setupdated(updated+1)
        // setSelected(selected); 
      }else {
        selected.push(id)
        setupdated(updated-1)
        // setSelected(selected); 
      }
      console.log("after" , id)
      console.log("after" , selected)
    };

    const handleChangePage = (newPage: number) => {
      setPage(newPage);
    };

    const handleChangeRowsPerPage = (event: React.ChangeEvent<HTMLInputElement>) => {
      setRowsPerPage(parseInt(event.target.value, 10));
      setPage(0);
    };
 

    const isSelected = (id: number) => selected.indexOf(id) !== -1;

    // Avoid a layout jump when reaching the last page with empty rows.
    const emptyRows =
      page > 0 ? Math.max(0, (1 + page) * rowsPerPage - data.length) : 0;
      function generateIndices(start, end) {
        return [...Array(end - start + 1).keys()].map(index => start + index);
      }

    function handleChangeDense(p_any: any) {
      throw new Error('Function not implemented.');
    }

    return (
      <Box sx={{ width: '100%' }}>
        <Paper sx={{ width: '100%', mb: 2 }}>
          <EnhancedTableToolbar numSelected={selected.length} name={name} checked={dense}
            onChange={handleChangeDense} />
          <TableContainer>
            <Table
              sx={{ minWidth: 750 }}
              aria-labelledby="tableTitle"
              size={dense ? 'small' : 'medium'}
            >
              <EnhancedTableHead
                numSelected={selected.length}
                order={order}
                orderBy={orderBy}
                onSelectAllClick={(  ) => {selected.length===0 ? 
                                          setSelected( generateIndices(0, data.length) ) : setSelected([])                                                  
                } }
                onRequestSort={handleRequestSort}
                rowCount={data.length}
                headCells={columns}
              />

              <TableBody>
                {data
                  ?.slice(page * rowsPerPage, page * rowsPerPage + rowsPerPage)
                  .map((row, index) => {
                    const isItemSelected = isSelected(row.id);
                    const labelId = `enhanced-table-checkbox-${index}`;


                    return (
                      <TableRow
                        hover
                        onClick={() => handleClick(row.id)}
                        role="checkbox"
                        aria-checked={isItemSelected}
                        tabIndex={-1}
                        selected={isItemSelected}
                        sx={{ cursor: 'pointer' }}
                      >
                        <TableCell padding="checkbox">
                          <Checkbox
                            color="primary"
                            checked={isItemSelected}
                            inputProps={{
                              'aria-labelledby': labelId,
                            }}
                          />
                        </TableCell>
                        {columns.map((column, c_index) => {
                          const value = row[column.id];

                          return (
                            <TableCell align={column.align} key={c_index + column.id + index}  >

                              {

                                column.format && (typeof (value) === 'number' || typeof (value) === 'boolean' || typeof value === 'object')
                                  ? column.format(value)
                                  : String(value)
                              }
                            </TableCell>
                          );
                        })
                        }

                      </TableRow>
                    );
                  })}
                {emptyRows > 0 && (
                  <TableRow
                    style={{
                      height: (dense ? 33 : 53) * emptyRows,
                    }}
                  >
                    <TableCell colSpan={6} />
                  </TableRow>
                )}
              </TableBody>
            </Table>
          </TableContainer>
          <TablePagination
            rowsPerPageOptions={[5, data.length]}
            component="div"
            count={data.length}
            rowsPerPage={rowsPerPage}
            page={page}
            onPageChange={handleChangePage}
            onRowsPerPageChange={handleChangeRowsPerPage}
          />
        </Paper>
      </Box>
    );
  }
}