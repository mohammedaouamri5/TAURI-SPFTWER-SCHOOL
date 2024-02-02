import { Paper, Table, TableBody, TableCell, TableContainer, TableHead, TablePagination, TableRow } from "@mui/material";
import { useEffect, useState } from "react";
import { LOG } from "./LOG";

export interface Column {
  id: string;
  label: string;
  minWidth?: number;
  align?: 'left' | 'right' | 'center';
  format?: (value: number) => string;
  // ((value: never) => string)  |
  // ((value: boolean) => string)  |
  // ((value: boolean | number) => string);
}

interface MyTableProps<T> {
  data: T[];
  columns: readonly Column[];
  name: string;
}



export function MyTable<T1>({ data, columns, name }: MyTableProps<T1>) {

  let rowsPerPageOptions = [10, 25, 100, Math.floor(data.length / 2), data.length].sort();
  rowsPerPageOptions.sort();
  rowsPerPageOptions = rowsPerPageOptions.filter(option => option > 0)
  rowsPerPageOptions.sort();

  const [page, setPage] = useState(0);
  const [rowsPerPage, setRowsPerPage] = useState(100);

  const handleChangePage = (_event: unknown, newPage: number) => {
    setPage(newPage);
  };

  const handleChangeRowsPerPage = (event: React.ChangeEvent<HTMLInputElement>) => {
    setRowsPerPage(+event.target.value);
    setPage(0);
  };

  useEffect(() => {
    if (data.length < page * rowsPerPage + rowsPerPage && data.length!= 0  ) {
      setRowsPerPage(Math.floor(data.length % rowsPerPage))
      const m_stat = { "rowsPerPage": rowsPerPage, "data.length": data.length, "page": page };
      LOG.info("fix the colume", "the RowsPerPage changed", m_stat);
    }else if (data.length == 0){
      setRowsPerPage(1)
      
    }

  })


  return (data.length ?
      <Paper sx = {{ width: '100%', overflow: 'hidden' } }>
        <TableContainer sx={{ maxHeight: 440 }}>
          <Table stickyHeader aria-label="sticky table">
            <TableHead>
              <TableRow>
                {columns.map((column) => (
                  <TableCell
                    
                    align={column.align}
                    style={{ minWidth: column.minWidth }}
                  >
                    {column.label}
                  </TableCell>
                ))}
              </TableRow>
            </TableHead>
            <TableBody>
              {
                (!(data.length + 1 < page * rowsPerPage + rowsPerPage)) ?
                  data
                    ?.slice(page * rowsPerPage, page * rowsPerPage + rowsPerPage)
                    ?.map((row: any) => {
                      return (
                        <TableRow hover role="checkbox" tabIndex={-1} >
                          {columns.map((column) => {
                            const value = row[column.id];

                            return (
                              <TableCell  align={column.align}>
                                {column.format && (typeof value === 'number' || typeof value === 'boolean')
                                  ? column.format(Number(value))
                                  : value
                                }
                              </TableCell>
                            );
                          })
                          }
                        </TableRow>
                      );
                    })
                  : null
              }
            </TableBody>
          </Table>
        </TableContainer>
        <TablePagination
          rowsPerPageOptions={rowsPerPageOptions}
          component="div"
          count={data.length}
          rowsPerPage={rowsPerPage}
          page={page}
          onPageChange={handleChangePage}
          onRowsPerPageChange={handleChangeRowsPerPage}
        />
      </Paper >
      : 
      <p>data is empty</p>);
            
  }
