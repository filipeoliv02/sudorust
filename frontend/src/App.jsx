import {useCallback, useState} from 'react';
import {
  Alert,
  Box,
  Button,
  CircularProgress,
  Container,
  Divider,
  Grid,
  Paper,
  TextField,
  Tooltip,
  Typography
} from '@mui/material';
import {createTheme, ThemeProvider} from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
import {Add, Bolt, Code} from '@mui/icons-material';
import SudokuGrid from './components/SudokuGrid';
import {BoardService} from './services/BoardService';

const theme = createTheme({
    palette: {
        primary: {
            main: '#1976d2',
        },
        secondary: {
            main: '#dc004e',
        },
    },
});

function App() {
    const [board, setBoard] = useState(null);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState('');
    const [size, setSize] = useState(9);
    const [clues, setClues] = useState(30);

    const handleGenerateBoard = useCallback(async () => {
        setLoading(true);
        setError('');
        try {
            const newBoard = await BoardService.generateBoard(size, clues);
            setBoard(newBoard);
        } catch (err) {
            setError('Failed to generate board: ' + err.message);
        } finally {
            setLoading(false);
        }
    }, [size, clues]);

    const handleSolveBoard = useCallback(async () => {
        if (!board) {
            setError('No board to solve');
            return;
        }

        setLoading(true);
        setError('');
        try {
            const solvedBoard = await BoardService.solveBoard(board);
            setBoard(solvedBoard);
        } catch (err) {
            setError('Failed to solve board: ' + err.message);
        } finally {
            setLoading(false);
        }
    }, [board]);

    const handleCellChange = useCallback((index, value) => {
        if (!board) return;

        const fixedCellIndex = board.fixed_cells.findIndex(cell => cell[0] === index);
        if (fixedCellIndex !== -1) return;

        const newCells = [...board.cells];
        newCells[index] = value;
        setBoard({...board, cells: newCells});
    }, [board]);

    return (
        <ThemeProvider theme={theme}>
            <CssBaseline/>
            <Container maxWidth="lg" sx={{py: 4}}>
                <Paper elevation={3} sx={{p: 4}}>
                    <Box textAlign="center" mb={4}>
                        <Typography variant="h3" component="h1" gutterBottom>
                            Sudoku Solver
                        </Typography>
                        <Typography variant="subtitle1" color="text.secondary">
                            Generate and solve Sudoku puzzles with Rust backend
                        </Typography>
                    </Box>

                    <Grid container spacing={3}>
                        <Grid item xs={12} md={4}>
                            <Paper elevation={2} sx={{p: 3}}>
                                <Typography variant="h6" gutterBottom>
                                    Controls
                                </Typography>
                                <Divider sx={{mb: 2}}/>

                                <Box mb={3}>
                                    <Typography variant="subtitle2" color="text.secondary" sx={{mb: 1}}>
                                        Board Size
                                    </Typography>
                                    <Box sx={{display: 'flex', gap: 1.5, mb: 2}}>
                                        <Tooltip title="4x4 grid">
                                            <Button
                                                variant={size === 4 ? 'contained' : 'outlined'}
                                                onClick={() => setSize(4)}
                                                sx={{
                                                    minWidth: 80,
                                                    height: 80,
                                                    borderRadius: 2,
                                                    fontSize: '0.8rem',
                                                    fontWeight: 'bold',
                                                    p: 0.5,
                                                    flex: 1
                                                }}
                                            >
                                                Small
                                            </Button>
                                        </Tooltip>
                                        <Tooltip title="9x9 grid">
                                            <Button
                                                variant={size === 9 ? 'contained' : 'outlined'}
                                                onClick={() => setSize(9)}
                                                sx={{
                                                    minWidth: 80,
                                                    height: 80,
                                                    borderRadius: 2,
                                                    fontSize: '0.8rem',
                                                    fontWeight: 'bold',
                                                    p: 0.5,
                                                    flex: 1
                                                }}
                                            >
                                                Normal
                                            </Button>
                                        </Tooltip>
                                        <Tooltip title="16x16 grid">
                                            <Button
                                                variant={size === 16 ? 'contained' : 'outlined'}
                                                onClick={() => setSize(16)}
                                                sx={{
                                                    minWidth: 80,
                                                    height: 80,
                                                    borderRadius: 2,
                                                    fontSize: '0.8rem',
                                                    fontWeight: 'bold',
                                                    p: 0.5,
                                                    flex: 1
                                                }}
                                            >
                                                Hard
                                            </Button>
                                        </Tooltip>
                                        <Tooltip title="25x25 grid">
                                            <Button
                                                variant={size === 25 ? 'contained' : 'outlined'}
                                                onClick={() => setSize(25)}
                                                sx={{
                                                    minWidth: 80,
                                                    height: 80,
                                                    borderRadius: 2,
                                                    fontSize: '0.8rem',
                                                    fontWeight: 'bold',
                                                    p: 0.5,
                                                    flex: 1
                                                }}
                                            >
                                                Extra Hard
                                            </Button>
                                        </Tooltip>
                                    </Box>
                                    <TextField
                                        fullWidth
                                        type="number"
                                        label="Number of Clues"
                                        value={clues}
                                        onChange={(e) => setClues(parseInt(e.target.value) || 30)}
                                        helperText="Fewer clues = harder puzzle"
                                        margin="normal"
                                        inputProps={{min: Math.ceil(size * size * 0.1), max: size * size}}
                                    />
                                </Box>

                                <Box display="flex" flexDirection="column" gap={2}>
                                    <Button
                                        variant="contained"
                                        startIcon={<Add/>}
                                        onClick={handleGenerateBoard}
                                        disabled={loading}
                                        fullWidth
                                    >
                                        {loading ? <CircularProgress size={20}/> : 'Generate New'}
                                    </Button>

                                    <Button
                                        variant="outlined"
                                        startIcon={<Bolt/>}
                                        onClick={handleSolveBoard}
                                        disabled={loading || !board}
                                        fullWidth
                                    >
                                        {loading ? <CircularProgress size={20}/> : 'Solve Current'}
                                    </Button>
                                </Box>

                                {error && (
                                    <Alert severity="error" sx={{mt: 2}}>
                                        {error}
                                    </Alert>
                                )}
                            </Paper>
                        </Grid>

                        <Grid item xs={12} md={8}>
                            <Paper elevation={2} sx={{p: 3}}>
                                <Typography variant="h6" gutterBottom>
                                    Sudoku Grid
                                </Typography>
                                <Divider sx={{mb: 2}}/>

                                {board ? (
                                    <SudokuGrid
                                        board={board}
                                        onCellChange={handleCellChange}
                                    />
                                ) : (
                                    <Box
                                        display="flex"
                                        flexDirection="column"
                                        alignItems="center"
                                        justifyContent="center"
                                        minHeight="400px"
                                        color="text.secondary"
                                    >
                                        <Code sx={{fontSize: 60, mb: 2, opacity: 0.3}}/>
                                        <Typography variant="h6">
                                            No board loaded
                                        </Typography>
                                        <Typography variant="body2">
                                            Click "Generate New" to start
                                        </Typography>
                                    </Box>
                                )}
                            </Paper>
                        </Grid>
                    </Grid>
                </Paper>
            </Container>
        </ThemeProvider>
    );
}

export default App;
