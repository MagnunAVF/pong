using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Graphics;
using Microsoft.Xna.Framework.Input;

namespace pong;

public class Game1 : Game
{
    private GraphicsDeviceManager _graphics;
    private SpriteBatch _spriteBatch;
    private Texture2D _pixelTexture;
    private SpriteFont _font;

    private Paddle _player1;
    private Paddle _player2;
    private Ball _ball;
    private ScoreManager _scoreManager;

    private const int ScreenWidth = 800;
    private const int ScreenHeight = 600;
    private const int PaddleWidth = 15;
    private const int PaddleHeight = 100;
    private const int BallSize = 15;
    private const int PaddleOffset = 30;

    private enum GameState { Playing, GameOver }
    private GameState _gameState;
    private KeyboardState _previousKeyboardState;

    public Game1()
    {
        _graphics = new GraphicsDeviceManager(this);
        Content.RootDirectory = "Content";
        IsMouseVisible = true;

        _graphics.PreferredBackBufferWidth = ScreenWidth;
        _graphics.PreferredBackBufferHeight = ScreenHeight;
    }

    protected override void Initialize()
    {
        _player1 = new Paddle(
            PaddleOffset,
            (ScreenHeight - PaddleHeight) / 2,
            PaddleWidth,
            PaddleHeight,
            ScreenHeight,
            Keys.W,
            Keys.S
        );

        _player2 = new Paddle(
            ScreenWidth - PaddleOffset - PaddleWidth,
            (ScreenHeight - PaddleHeight) / 2,
            PaddleWidth,
            PaddleHeight,
            ScreenHeight,
            Keys.Up,
            Keys.Down
        );

        _ball = new Ball(
            (ScreenWidth - BallSize) / 2,
            (ScreenHeight - BallSize) / 2,
            BallSize,
            ScreenWidth,
            ScreenHeight
        );

        _scoreManager = new ScoreManager();
        _gameState = GameState.Playing;
        _previousKeyboardState = Keyboard.GetState();

        base.Initialize();
    }

    protected override void LoadContent()
    {
        _spriteBatch = new SpriteBatch(GraphicsDevice);

        _pixelTexture = new Texture2D(GraphicsDevice, 1, 1);
        _pixelTexture.SetData(new[] { Color.White });

        _font = Content.Load<SpriteFont>("Font");
    }

    protected override void Update(GameTime gameTime)
    {
        var keyboardState = Keyboard.GetState();

        if (keyboardState.IsKeyDown(Keys.Escape))
            Exit();

        if (_gameState == GameState.Playing)
        {
            _player1.Update(gameTime);
            _player2.Update(gameTime);
            _ball.Update(gameTime);

            _ball.CheckPaddleCollision(_player1);
            _ball.CheckPaddleCollision(_player2);

            if (_ball.IsOutOfBounds())
            {
                if (_ball.ScoredOnLeft())
                    _scoreManager.IncrementPlayer2Score();
                else
                    _scoreManager.IncrementPlayer1Score();

                if (_scoreManager.HasWinner())
                {
                    _gameState = GameState.GameOver;
                }
                else
                {
                    _ball.Reset();
                }
            }
        }
        else if (_gameState == GameState.GameOver)
        {
            if (keyboardState.IsKeyDown(Keys.Space) && _previousKeyboardState.IsKeyUp(Keys.Space))
            {
                _scoreManager.Reset();
                _ball.Reset();
                _player1.Reset(PaddleOffset, (ScreenHeight - PaddleHeight) / 2);
                _player2.Reset(ScreenWidth - PaddleOffset - PaddleWidth, (ScreenHeight - PaddleHeight) / 2);
                _gameState = GameState.Playing;
            }
        }

        _previousKeyboardState = keyboardState;
        base.Update(gameTime);
    }

    protected override void Draw(GameTime gameTime)
    {
        GraphicsDevice.Clear(Color.Black);

        _spriteBatch.Begin();

        DrawCenterLine();

        _player1.Draw(_spriteBatch, _pixelTexture);
        _player2.Draw(_spriteBatch, _pixelTexture);
        _ball.Draw(_spriteBatch, _pixelTexture);

        _scoreManager.Draw(_spriteBatch, _font, ScreenWidth, ScreenHeight);

        if (_gameState == GameState.GameOver)
        {
            string winnerText = _scoreManager.GetWinnerText();
            Vector2 winnerSize = _font.MeasureString(winnerText);
            Vector2 winnerPos = new Vector2(
                (ScreenWidth - winnerSize.X) / 2,
                (ScreenHeight - winnerSize.Y) / 2
            );
            _spriteBatch.DrawString(_font, winnerText, winnerPos, Color.Yellow);

            string restartText = "Press SPACE to restart";
            Vector2 restartSize = _font.MeasureString(restartText);
            Vector2 restartPos = new Vector2(
                (ScreenWidth - restartSize.X) / 2,
                (ScreenHeight - restartSize.Y) / 2 + 40
            );
            _spriteBatch.DrawString(_font, restartText, restartPos, Color.White);
        }

        _spriteBatch.End();

        base.Draw(gameTime);
    }

    private void DrawCenterLine()
    {
        int dashHeight = 10;
        int dashGap = 10;
        int lineWidth = 2;
        int centerX = ScreenWidth / 2 - lineWidth / 2;

        for (int y = 0; y < ScreenHeight; y += dashHeight + dashGap)
        {
            Rectangle dash = new Rectangle(centerX, y, lineWidth, dashHeight);
            _spriteBatch.Draw(_pixelTexture, dash, Color.White);
        }
    }
}
