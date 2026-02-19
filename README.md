# Pong Game - MonoGame

A classic Pong game implementation using the MonoGame framework.

## About

This is a recreation of the classic Pong arcade game built with MonoGame. The project demonstrates fundamental game development concepts including sprite rendering, collision detection, input handling, and game state management.

## Pong Game Controls

### Player 1 (Left Paddle)

- **W** - Move paddle up
- **S** - Move paddle down

### Player 2 (Right Paddle)

- **Up Arrow** - Move paddle up
- **Down Arrow** - Move paddle down

### General Controls

- **ESC** - Exit game
- **SPACE** - Restart game (when game over)

### Game Rules

- First player to reach **11 points** wins
- Ball speeds up slightly with each paddle hit
- Ball angle changes based on where it hits the paddle
- Score appears at the top of the screen

## Prerequisites

- [.NET 9.0 SDK](https://dotnet.microsoft.com/download) or later
- MonoGame 3.8.1 (installed via NuGet)
- MonoGame Content Builder Tool (dotnet-mgcb)
- GTK+3 (required for MonoGame Content Builder Editor on macOS/Linux)

## Getting Started

### Installation

1. Clone the repository:

```bash
git clone <repository-url>
cd pong
```

2. Restore dependencies:

```bash
dotnet restore
```

3. Install MonoGame Content Builder Tool (if not already installed):

```bash
cd pong && dotnet tool restore
```

4. Install GTK+3 (macOS only, required for Content Editor):

```bash
brew install gtk+3
```

**Note**: On Linux, install GTK+3 using your package manager:

- Ubuntu/Debian: `sudo apt-get install libgtk-3-0`
- Fedora: `sudo dnf install gtk3`

### Building

```bash
dotnet build
```

### Running

```bash
dotnet run --project pong
```

## Development

### Adding Content

1. Open the Content Pipeline Tool:

```bash
cd pong && dotnet mgcb-editor Content/Content.mgcb
```

2. Add your assets (sprites, fonts, sounds, etc.)
3. Build the content pipeline
4. Reference the assets in your code

### Building for Release

```bash
dotnet publish pong/pong.csproj -c Release -r <runtime-identifier> --self-contained

# macos example
dotnet publish pong/pong.csproj -c Release -r osx-x64 --self-contained
```

Common runtime identifiers:

- `win-x64` - Windows 64-bit
- `osx-x64` - macOS 64-bit
- `linux-x64` - Linux 64-bit

### Running the Published Build

After building, run the executable:

```bash
# macOS/Linux
./pong/bin/Release/net9.0/osx-x64/publish/pong

# Windows
./pong\bin\Release\net9.0\win-x64\publish\pong.exe
```

## Troubleshooting

### MonoGame Content Builder Editor won't open

**Error**: "Could not execute because the specified command or file was not found"

**Solution**: Install GTK+3 which is required for the GUI editor:

```bash
# macOS
brew install gtk+3

# Ubuntu/Debian
sudo apt-get install libgtk-3-0

# Fedora
sudo dnf install gtk3
```

After installation, the `mgcb-editor` command should work properly.

### Alternative: Manual Content Management

If you prefer not to use the GUI editor, you can:

1. **Edit Content.mgcb manually** - Add content references directly to the file
2. **Use command-line mgcb tool** - Build content from the command line
3. **Generate textures in code** - For simple games like Pong, create graphics programmatically

Example of adding a texture manually to `Content.mgcb`:

```
#begin MyTexture.png
/importer:TextureImporter
/processor:TextureProcessor
/processorParam:ColorKeyColor=255,0,255,255
/processorParam:ColorKeyEnabled=True
/processorParam:GenerateMipmaps=False
/processorParam:PremultiplyAlpha=True
/processorParam:ResizeToPowerOfTwo=False
/processorParam:MakeSquare=False
/processorParam:TextureFormat=Color
/build:MyTexture.png
```

### Build Errors

**Error**: "Cannot find a manifest file" or "dotnet-mgcb does not exist"

**Solution**: Install the tool locally in the project:

```bash
cd pong
dotnet new tool-manifest
dotnet tool install dotnet-mgcb --local
dotnet tool restore
```

### Published executable requires .NET runtime

**Error**: "You must install .NET to run this application"

**Solution**: Use the `--self-contained` flag when publishing:

```bash
dotnet publish pong/pong.csproj -c Release -r osx-x64 --self-contained
```

This creates a standalone executable that includes the .NET runtime (~65MB larger).
