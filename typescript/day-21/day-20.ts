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
