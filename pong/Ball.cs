using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Graphics;
using System;

namespace pong
{
    public class Ball
    {
        private const float BaseSpeed = 300f;
        private const float SpeedIncreaseMultiplier = 1.05f;
        private const float MaxAngleRadians = MathHelper.PiOver4;
        private const float AngleVariation = MathHelper.PiOver4 / 2;

        public Rectangle Bounds { get; private set; }
        public Vector2 Velocity { get; set; }

        private readonly int _screenWidth;
        private readonly int _screenHeight;
        private readonly Random _random;

        public Ball(int x, int y, int size, int screenWidth, int screenHeight)
        {
            Bounds = new Rectangle(x, y, size, size);
            _screenWidth = screenWidth;
            _screenHeight = screenHeight;
            _random = new Random();
            Reset();
        }

        public void Reset()
        {
            ResetPosition();
            ResetVelocity();
        }

        private void ResetPosition()
        {
            int centerX = (_screenWidth - Bounds.Width) / 2;
            int centerY = (_screenHeight - Bounds.Height) / 2;
            Bounds = new Rectangle(centerX, centerY, Bounds.Width, Bounds.Height);
        }

        private void ResetVelocity()
        {
            float angle = GetRandomAngle();
            int direction = GetRandomDirection();

            Velocity = new Vector2(
                MathF.Cos(angle) * BaseSpeed * direction,
                MathF.Sin(angle) * BaseSpeed
            );
        }

        private float GetRandomAngle()
        {
            return (float)(_random.NextDouble() * AngleVariation - AngleVariation / 2);
        }

        private int GetRandomDirection()
        {
            return _random.Next(2) == 0 ? -1 : 1;
        }

        public void Update(GameTime gameTime)
        {
            float deltaTime = (float)gameTime.ElapsedGameTime.TotalSeconds;
            UpdatePosition(deltaTime);
            HandleWallCollision();
        }

        private void UpdatePosition(float deltaTime)
        {
            int newX = Bounds.X + (int)(Velocity.X * deltaTime);
            int newY = Bounds.Y + (int)(Velocity.Y * deltaTime);

            Bounds = new Rectangle(newX, newY, Bounds.Width, Bounds.Height);
        }

        private void HandleWallCollision()
        {
            bool hitTopWall = Bounds.Y <= 0;
            bool hitBottomWall = Bounds.Y + Bounds.Height >= _screenHeight;

            if (hitTopWall || hitBottomWall)
            {
                Velocity = new Vector2(Velocity.X, -Velocity.Y);

                int newY = hitTopWall ? 0 : _screenHeight - Bounds.Height;
                Bounds = new Rectangle(Bounds.X, newY, Bounds.Width, Bounds.Height);
            }
        }

        public void Draw(SpriteBatch spriteBatch, Texture2D texture)
        {
            spriteBatch.Draw(texture, Bounds, Color.White);
        }

        public bool CheckPaddleCollision(Paddle paddle)
        {
            if (!Bounds.Intersects(paddle.Bounds))
                return false;

            float bounceAngle = CalculateBounceAngle(paddle);
            float newSpeed = CalculateNewSpeed();
            int direction = GetBounceDirection();

            UpdateVelocityAfterCollision(bounceAngle, newSpeed, direction);
            RepositionAfterPaddleCollision(paddle);

            return true;
        }

        private float CalculateBounceAngle(Paddle paddle)
        {
            int paddleCenterY = paddle.Bounds.Y + paddle.Bounds.Height / 2;
            int ballCenterY = Bounds.Y + Bounds.Height / 2;

            float relativeIntersectY = paddleCenterY - ballCenterY;
            float normalizedIntersectY = relativeIntersectY / (paddle.Bounds.Height / 2);

            return normalizedIntersectY * MaxAngleRadians;
        }

        private float CalculateNewSpeed()
        {
            return Velocity.Length() * SpeedIncreaseMultiplier;
        }

        private int GetBounceDirection()
        {
            return Velocity.X > 0 ? -1 : 1;
        }

        private void UpdateVelocityAfterCollision(float bounceAngle, float speed, int direction)
        {
            Velocity = new Vector2(
                MathF.Cos(bounceAngle) * speed * direction,
                -MathF.Sin(bounceAngle) * speed
            );
        }

        private void RepositionAfterPaddleCollision(Paddle paddle)
        {
            int newX = Velocity.X > 0
                ? paddle.Bounds.Right
                : paddle.Bounds.Left - Bounds.Width;

            Bounds = new Rectangle(newX, Bounds.Y, Bounds.Width, Bounds.Height);
        }

        public bool IsOutOfBounds()
        {
            return Bounds.X < -Bounds.Width || Bounds.X > _screenWidth;
        }

        public bool ScoredOnLeft()
        {
            return Bounds.X < -Bounds.Width;
        }
    }
}
