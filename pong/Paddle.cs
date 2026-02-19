using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Graphics;
using Microsoft.Xna.Framework.Input;

namespace pong
{
    public class Paddle
    {
        private const float DefaultSpeed = 400f;
        private const int MinYPosition = 0;

        public Rectangle Bounds { get; private set; }
        public float Speed { get; set; }

        private readonly int _screenHeight;
        private readonly Keys _upKey;
        private readonly Keys _downKey;

        public Paddle(int x, int y, int width, int height, int screenHeight, Keys upKey, Keys downKey)
        {
            Bounds = new Rectangle(x, y, width, height);
            Speed = DefaultSpeed;
            _screenHeight = screenHeight;
            _upKey = upKey;
            _downKey = downKey;
        }

        public void Update(GameTime gameTime)
        {
            KeyboardState keyboardState = Keyboard.GetState();
            float deltaTime = (float)gameTime.ElapsedGameTime.TotalSeconds;

            int movement = GetMovementInput(keyboardState, deltaTime);
            int newY = CalculateNewYPosition(movement);

            UpdatePosition(newY);
        }

        private int GetMovementInput(KeyboardState keyboardState, float deltaTime)
        {
            int movement = 0;
            int movementAmount = (int)(Speed * deltaTime);

            if (keyboardState.IsKeyDown(_upKey))
            {
                movement -= movementAmount;
            }

            if (keyboardState.IsKeyDown(_downKey))
            {
                movement += movementAmount;
            }

            return movement;
        }

        private int CalculateNewYPosition(int movement)
        {
            int newY = Bounds.Y + movement;
            int maxYPosition = _screenHeight - Bounds.Height;

            return MathHelper.Clamp(newY, MinYPosition, maxYPosition);
        }

        private void UpdatePosition(int newY)
        {
            Bounds = new Rectangle(Bounds.X, newY, Bounds.Width, Bounds.Height);
        }

        public void Draw(SpriteBatch spriteBatch, Texture2D texture)
        {
            spriteBatch.Draw(texture, Bounds, Color.White);
        }

        public void Reset(int x, int y)
        {
            Bounds = new Rectangle(x, y, Bounds.Width, Bounds.Height);
        }

        public int GetCenterY()
        {
            return Bounds.Y + Bounds.Height / 2;
        }
    }
}
