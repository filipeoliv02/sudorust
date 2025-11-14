const API_BASE_URL = 'http://localhost:3000';

export class BoardService {
    static async generateBoard(size, clues) {
        try {
            const response = await fetch(`${API_BASE_URL}/generate`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({size, clues}),
            });

            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }

            return await response.json();
        } catch (error) {
            console.error('Failed to generate board:', error);
            throw new Error(`Failed to generate board: ${error.message}`);
        }
    }

    static async solveBoard(board) {
        try {
            const response = await fetch(`${API_BASE_URL}/solve`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(board),
            });

            if (!response.ok) {
                if (response.status === 400) {
                    throw new Error('No solution found for this board');
                }
                throw new Error(`HTTP error! status: ${response.status}`);
            }

            return await response.json();
        } catch (error) {
            console.error('Failed to solve board:', error);
            throw new Error(`Failed to solve board: ${error.message}`);
        }
    }
}