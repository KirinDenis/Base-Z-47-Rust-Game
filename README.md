# Mystery of Base Z-47

**Mystery of Base Z-47** is a retro-style adventure game being developed in Assembly language, specifically designed to run on DOSBOX with VGA support. This project is an homage to classic DOS games, combining challenging gameplay with atmospheric storytelling.

## Overview

In "Mystery of Base Z-47," players will explore an abandoned space base on an unknown planet. They will face various challenges, including puzzles, traps, and lurking dangers as they attempt to uncover the secrets hidden within the base. The game is built from the ground up using Assembly language and will be compiled with TASM (Turbo Assembler) version 3.2.

### Key Features
- **Classic DOS Aesthetic**: True to the era, with 256-color VGA graphics.
- **Challenging Gameplay**: Puzzles, traps, and enemies requiring both strategy and quick reflexes.
- **Assembly Language**: Handcrafted code designed to maximize performance and create an authentic retro experience.
- **Sound and Music**: Sound effects and music created for the Sound Blaster, enhancing the game's atmosphere.

## Tools and Collaborators

This project is developed using the following tools and collaborators:

- **Turbo Assembler (TASM) 3.2**: The primary assembler for compiling the game.
- **DOSBOX**: Used for testing and running the game on modern systems.
- **ChatGPT (OpenAI)**: Assisting in planning, writing, and debugging the Assembly code, as well as generating ideas and solutions for game mechanics.
- **MidJourney**: Employed for generating concept art and visual assets for the game.
- **AUDIO**: Used for creating sound effects and background music, helping to set the mood and immerse players in the game world.

## Installation

To play **Mystery of Base Z-47**, follow these steps:

1. **Download DOSBOX**: Install the latest version of DOSBOX from the [official site](https://www.dosbox.com/).
2. **Clone the Repository**: 
    ```sh
    git clone https://github.com/yourusername/mystery-of-base-z47.git
    ```
3. **Compile the Game**:
    - Open DOSBOX.
    - Mount the game directory.
    - Run the TASM assembler to compile the source code:
      ```sh
      tasm /zi main.asm
      tlink /v main.obj
      ```
    - Run the executable:
      ```sh
      basez47.exe
      ```

## Development

The game is in the early stages of development, with ongoing work on core mechanics, graphics, and sound. Contributions and suggestions are welcome.

### How to Contribute

- **Code**: Fork the repository, make your changes, and submit a pull request.
- **Art**: If you're skilled in pixel art or retro game design, your contributions are welcome.
- **Sound/Music**: If you can create authentic retro soundtracks or effects, feel free to collaborate.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.

## Contact

For any questions, suggestions, or collaborations, feel free to contact the project maintainer at [deniskirinacs@gmail.com].

---

### Acknowledgments

Special thanks to the following tools and platforms that made this project possible:

- **ChatGPT by OpenAI**: For providing assistance with code, design ideas, and project planning.
- **MidJourney**: For generating the visual concept art that inspired much of the game's look and feel.
- **AUDIO**: For sound design and music composition tools that bring the game's atmosphere to life.

---

**Note:** This game is a work in progress. Stay tuned for updates and new releases as development continues.

---

