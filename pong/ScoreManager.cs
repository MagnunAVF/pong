using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Graphics;

namespace pong
{
    public class ScoreManager
    {
        private const int DefaultWinningScore = 11;
        private const int ScoreDisplayYPosition = 20;
        private const float ScoreDisplayQuarterPosition = 0.25f;
        private const float ScoreDisplayThreeQuarterPosition = 0.75f;

        public int Player1Score { get; private set; }
        public int Player2Score { get; private set; }
        public int WinningScore { get; set; }

        public ScoreManager(int winningScore = DefaultWinningScore)
        {
            WinningScore = winningScore;
            Reset();
        }

        public void IncrementPlayer1Score()
        {
            Player1Score++;
        }

        public void IncrementPlayer2Score()
        {
            Player2Score++;
        }

        public bool HasWinner()
        {
            return Player1Score >= WinningScore || Player2Score >= WinningScore;
        }

        public string GetWinnerText()
        {
            if (Player1Score >= WinningScore)
                return "Player 1 Wins!";

            if (Player2Score >= WinningScore)
                return "Player 2 Wins!";

            return string.Empty;
        }

        public void Reset()
        {
            Player1Score = 0;
            Player2Score = 0;
        }

        public void Draw(SpriteBatch spriteBatch, SpriteFont font, int screenWidth, int screenHeight)
        {
            DrawPlayerScore(spriteBatch, font, Player1Score, screenWidth, isPlayer1: true);
            DrawPlayerScore(spriteBatch, font, Player2Score, screenWidth, isPlayer1: false);
        }

        private void DrawPlayerScore(SpriteBatch spriteBatch, SpriteFont font, int score, int screenWidth, bool isPlayer1)
        {
            string scoreText = score.ToString();
            Vector2 scoreSize = font.MeasureString(scoreText);

            float horizontalPosition = isPlayer1
                ? screenWidth * ScoreDisplayQuarterPosition
                : screenWidth * ScoreDisplayThreeQuarterPosition;

            Vector2 scorePosition = new Vector2(
                horizontalPosition - scoreSize.X / 2,
                ScoreDisplayYPosition
            );

            spriteBatch.DrawString(font, scoreText, scorePosition, Color.White);
        }
    }
}
