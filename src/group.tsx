
import * as React from 'react';
import {
  Paper,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TablePagination,
  TableRow
} from '@mui/material';

import { User } from "./user";
import { invoke } from '@tauri-apps/api/tauri';
import { useState } from 'react';
import { render } from 'react-dom';




export interface Group {
  date_end: string;
  date_start: string;
  id: number;
  idfrom: number;
  idteacher: number;
  idto: number;
  name: String
  is_done: boolean;
}


interface All_The_Groups_TableProps {
  data: Group[];
}



interface Column {
  id: 'date_end' | 'date_start' | 'id' | 'idfrom' | 'idteacher' | 'idto' | 'is_done';
  label: string;
  minWidth?: number;
  align?: 'center';
  format?: (value: number) => string;
}

interface UserIdGroup {
  [key: string]: User;
}

var users: UserIdGroup = {};

export default function All_The_Groups_Table({ data }: All_The_Groups_TableProps) {
  const [page, setPage] = React.useState(0);
  const [rowsPerPage, setRowsPerPage] = React.useState(10);
  const [users, setUsers] = useState<UserIdGroup>({});

  const setUserData = async (element: Group) => {
    try {
      const m_user: User = await invoke<User>("get_user_by_id", { id: element.id });

      setUsers((prevUsers) => ({
        ...prevUsers,
        [`${element.id}`]: new User(
          m_user.id,
          m_user.id_type,
          m_user.name,
          m_user.family_name,
          m_user.birth_day,
          m_user.notes
        )
      }));

    } catch (error) {
      console.error("Error fetching user:", error);
    }
  };

  React.useEffect(() => {
    if (data.length > 0 && Object.keys(users).length === 0) {
      data.forEach((element) => {
        setUserData(element);
      });
    }
  }, [data, users]);

  console.log(users)
  const handleChangePage = (event: unknown, newPage: number) => {
    setPage(newPage);
  };



  const handleChangeRowsPerPage = (event: React.ChangeEvent<HTMLInputElement>) => {
    setRowsPerPage(+event.target.value);
    setPage(0);
  };


  const columns: readonly Column[] = [
    {
      id: 'date_end',
      label: 'until',
      align: 'center',
      minWidth: 120
    },
    {
      id: 'date_start',
      label: 'started',
      align: 'center',
      minWidth: 90
    },
    {
      id: 'idteacher',
      label: 'by',
      minWidth: 100,
      align: 'center',
      format: (value: number) => {

        // return `${value}`;
        return users[`${value}`]?.name || "bruh";
      },
    },
    {
      id: 'idfrom',
      label: 'from',
      minWidth: 100,
      align: 'center',
      format: (value: number) => value.toLocaleString('en-US'),
    },
    {
      id: 'idto',
      label: 'to',
      minWidth: 100,
      align: 'center',
      format: (value: number) => value.toLocaleString('en-US'),
    },
    {
      id: 'is_done',
      label: 'working',
      minWidth: 70,
      align: 'center',
      format: (value: number) => { if (value == 0) { return "✅" } else { return "❌" } },
    },
  ];

  // return <p>fuck you</p>;

  return (
    <Paper sx={{ width: '100%', overflow: 'hidden' }}>
      <TableContainer sx={{ maxHeight: 440 }}>
        <Table stickyHeader aria-label="sticky table">
          <TableHead>
            <TableRow>
              {columns.map((column) => (
                <TableCell
                  key={column.id}
                  align={column.align}
                  style={{ minWidth: column.minWidth }}
                >
                  {column.label}
                </TableCell>
              ))}
            </TableRow>
          </TableHead>
          <TableBody>
            {data
              ?.slice(page * rowsPerPage, page * rowsPerPage + rowsPerPage)
              ?.map((row: Group) => {
                return (
                  <TableRow hover role="checkbox" tabIndex={-1} key={row.id}>
                    {columns.map((column) => {
                      const value = row[column.id];

                      return (
                        <TableCell key={column.id} align={column.align}>
                          {column.format && (typeof value === 'number' || typeof value ===   'boolean')
                            ? column.format(Number(value))
                            : value}
                        </TableCell>
                      );
                    })}
                  </TableRow>
                );
              })}
          </TableBody>
        </Table>
      </TableContainer>
      <TablePagination
        rowsPerPageOptions={[10, 25, 100, data.length]}
        component="div"
        count={data.length}
        rowsPerPage={rowsPerPage}
        page={page}
        onPageChange={handleChangePage}
        onRowsPerPageChange={handleChangeRowsPerPage}
      />
    </Paper>
  );



}



