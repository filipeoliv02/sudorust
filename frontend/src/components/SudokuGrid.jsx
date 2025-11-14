import {useState} from 'react';
import {Box, Paper, Table, TableBody, TableCell, TableContainer, TableRow, TextField} from '@mui/material';

const SudokuGrid = ({board, onCellChange}) => {
    const [selectedCell, setSelectedCell] = useState(null);
    const boxSize = Math.sqrt(board.size);

    const isFixedCell = (index) => {
        return board.fixed_cells.some(cell => cell[0] === index);
    };

    const handleCellClick = (index) => {
        if (!isFixedCell(index)) {
            setSelectedCell(index);
        }
    };

    const handleCellChange = (index, value) => {
        if (value === '' || (value >= '1' && value <= board.size.toString())) {
            onCellChange(index, value === '' ? 0 : parseInt(value));
        }
    };

    const renderCell = (index) => {
        const row = Math.floor(index / board.size);
        const col = index % board.size;
        const value = board.cells[index];
        const fixed = isFixedCell(index);
        const isSelected = selectedCell === index;
        const boxRow = Math.floor(row / boxSize);
        const boxCol = Math.floor(col / boxSize);

        const borderTop = row % boxSize === 0 ? '2px solid #1976d2' : '1px solid #ddd';
        const borderLeft = col % boxSize === 0 ? '2px solid #1976d2' : '1px solid #ddd';
        const borderRight = (col + 1) % boxSize === 0 ? '2px solid #1976d2' : (col === board.size - 1 ? '2px solid #1976d2' : '1px solid #ddd');
        const borderBottom = (row + 1) % boxSize === 0 ? '2px solid #1976d2' : (row === board.size - 1 ? '2px solid #1976d2' : '1px solid #ddd');

        return (
            <TableCell
                key={index}
                sx={{
                    width: 40,
                    height: 40,
                    p: 0,
                    borderTop,
                    borderLeft,
                    borderRight,
                    borderBottom,
                    cursor: fixed ? 'default' : 'pointer',
                    backgroundColor: isSelected ? '#e3f2fd' : 'transparent',
                    '&:hover': {
                        backgroundColor: fixed ? 'transparent' : '#f5f5f5',
                    },
                }}
                onClick={() => handleCellClick(index)}
            >
                {fixed ? (
                    <Box
                        sx={{
                            width: '100%',
                            height: '100%',
                            display: 'flex',
                            alignItems: 'center',
                            justifyContent: 'center',
                            fontWeight: 'bold',
                            color: '#1976d2',
                        }}
                    >
                        {value === 0 ? '' : value}
                    </Box>
                ) : (
                    <TextField
                        variant="standard"
                        value={value === 0 ? '' : value}
                        onChange={(e) => handleCellChange(index, e.target.value)}
                        onFocus={() => setSelectedCell(index)}
                        sx={{
                            '& .MuiInputBase-input': {
                                textAlign: 'center',
                                fontSize: '1.1rem',
                                fontWeight: 'bold',
                                color: '#333',
                            },
                            '& .MuiInput-underline:before': {
                                borderBottom: 'none',
                            },
                            '& .MuiInput-underline:after': {
                                borderBottom: '2px solid #1976d2',
                            },
                        }}
                        inputProps={{
                            style: {
                                width: '30px',
                                height: '30px',
                                textAlign: 'center',
                                border: 'none',
                                outline: 'none',
                            },
                        }}
                    />
                )}
            </TableCell>
        );
    };

    const cells = [];
    for (let i = 0; i < board.size * board.size; i++) {
        cells.push(renderCell(i));
    }

    const rows = [];
    for (let row = 0; row < board.size; row++) {
        const rowCells = cells.slice(row * board.size, (row + 1) * board.size);
        rows.push(
            <TableRow key={row}>
                {rowCells}
            </TableRow>
        );
    }

    return (
        <Box display="flex" justifyContent="center" p={2}>
            <TableContainer component={Paper} elevation={1}>
                <Table sx={{borderCollapse: 'collapse'}}>
                    <TableBody>
                        {rows}
                    </TableBody>
                </Table>
            </TableContainer>
        </Box>
    );
};

export default SudokuGrid;