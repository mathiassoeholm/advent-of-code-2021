export function naiveSolution(
  player1StartPos: number,
  player2StartPos: number
) {
  let stats = {
    player1Score: 0,
    player2Score: 0,
    player1Pos: player1StartPos - 1,
    player2Pos: player2StartPos - 1,
  };

  let numDieRolls = 0;
  let turn: 1 | 2 = 1;

  const rollDie = () => {
    numDieRolls++;
    return ((numDieRolls - 1) % 100) + 1;
  };

  while (stats.player1Score < 1000 && stats.player2Score < 1000) {
    for (let i = 0; i < 3; i++) {
      stats[`player${turn}Pos`] = (stats[`player${turn}Pos`] + rollDie()) % 10;
    }
    stats[`player${turn}Score`] += stats[`player${turn}Pos`] + 1;
    turn = turn === 1 ? 2 : 1;
  }

  return Math.min(stats.player1Score, stats.player2Score) * numDieRolls;
}

type Game = [
  p1Pos: number,
  p2Pos: number,
  p1Score: number,
  p2Score: number,
  turn: number, // 0..5
  acc: number
];

export function part2(player1StartPos: number, player2StartPos: number) {
  const games = new Map<string, number>();

  const encodeGame = (game: Game) => game.join(",");
  const decodeGame = (str: string) =>
    str.split(",").map((v) => parseInt(v)) as Game;

  const rollDie = (
    roll: number,
    [p1Pos, p2Pos, p1Score, p2Score, turn, acc]: Game
  ): Game => {
    const pos = turn <= 2 ? p1Pos : p2Pos;
    const newPos = ((pos + (roll + acc) - 1) % 10) + 1;

    if (turn <= 2) {
      return [
        newPos,
        p2Pos,
        turn === 2 ? p1Score + newPos : p1Score,
        p2Score,
        (turn + 1) % 6,
        turn === 2 ? 0 : acc + roll,
      ];
    } else {
      return [
        p1Pos,
        p2Pos,
        p1Score,
        turn === 5 ? p2Score + newPos : p2Score,
        (turn + 1) % 6,
        turn === 5 ? 0 : acc + roll,
      ];
    }
  };

  games.set(encodeGame([player1StartPos, player2StartPos, 0, 0, 0, 0]), 1);

  console.log(
    games.get(encodeGame([player1StartPos, player2StartPos, 0, 0, 0, 0]))
  );

  let gameStr: string;
  let gamesWonByPlayer1 = 0;
  let gamesWonByPlayer2 = 0;
  while ((gameStr = [...games.keys()][0])) {
    const game = decodeGame(gameStr);
    const numGames = games.get(encodeGame(game))!;

    for (let roll = 1; roll <= 3; roll++) {
      const newGame = rollDie(roll, game);
      const currentNumber = games.get(encodeGame(newGame)) ?? 0;
      if (newGame[2] < 21 && newGame[3] < 21) {
        games.set(encodeGame(newGame), currentNumber + numGames);
      } else {
        // Some player won
        if (newGame[2] > newGame[3]) {
          gamesWonByPlayer1 += currentNumber + numGames;
        } else if (newGame[3] > newGame[2]) {
          gamesWonByPlayer2 += currentNumber + numGames;
        }
      }
    }

    games.delete(encodeGame(game));
  }

  console.log("Player 1 won: ", gamesWonByPlayer1);
  console.log("Player 2 won: ", gamesWonByPlayer2);
}
