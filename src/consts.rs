pub const EASTEREGG: &str = "
                                            :
                                           :::
                    '::                   ::::
                    '::::.     .....:::.:::::::
                    '::::::::::::::::::::::::::::
                    ::::::XUWWWWWU:::::XW$$$$$$WX:
                    ::::X$$$$$$$$$$W::X$$$$$$$$$$Wh
                   ::::t$$$$$$$$$$$$W:$$$$$$P*$$$$M::
                   :::X$$$$$$''''$$$$X$$$$$   ^$$$$X:::
                  ::::M$$$$$$    ^$$$RM$$$L    <$$$X::::
                .:::::M$$$$$$     $$$R:$$$$.   d$$R:::`
               '~::::::?$$$$$$...d$$$X$6R$$$$$$$$RXW$X:'`
                 '~:WNWUXT#$$$$$$$$TU$$$$W6IBBIW@$$RX:
";

pub const EASTEREGG2: &str = "
                    +T+++++++T+++
                    +++|+++++++|+++|
                    ++++++++++++++/+
                    ++++++++++++++/++
                    +++++++++++++/++++
                    +++++:::iiiII/++++.
                    IIIIIIIIIIII/Ii++++
                    ITTTTTTTTTT/III++++.
                    'IIIIIIIIIIiIIII++++
                      'IIIIIIIIIiIIII+++.
                        'IIIIIIIIiIIIi+++
                          'IIIIIIIiIII+++
                            'I/\\IIIiIII++
                             ///\\IIIiIIi+
              .o8OOOOOOOOOOOOOo/IIIIIiIIi
            oOOOOOOOOOOOOOOOOOOOOIIIIIiI
           888888888888OOOOOOOOOOOOIIIII
      o8OOOOOOOOOOOOOOO88888OOOOOOOOIIIT
    oOOOOOOOOOOOOOOOOOOOOOOO888OOOOOb
   8OOOOOOOOOOOOOOOOOOOOOOOOOOO8OOOOOb
   OOOOOOOOOOOOOOOOOOOOOOOOOOOOO88OOOOb
   OOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO8OOOO.
   OOOO~~~~~~~~~~~~~~~~OOOOOOOOOOOO8OOOb
   OO~ oo8888888888888oo ~~OOOOOOOOO8OOO
   8 oO888888888888888888oo ~~OOOOOOO8OO
   8 OO888888888888888888888oo OOOOOO8OO
   ~8 O8888888888888888888888888 ~OOOOOP
     ~O88 ~888~Y88Y88P    888888>-~OOOO
       88b ~~ .888 ~~    d88888| ( OOOO
        88Xooo88888ooood8888888/ /XooO~
         ~88888888888888888888| |OOX|
         //88888888888888888:',:OOO|
       ,'/OOOO888888888888~<-
       |   Y ..  ,O\\******    _ / | |<_
     ___~-'8o8OO88OX8OOOO    ~ Y  '/O8|
   ,'   `-_OOOOOOOXX|OOOO8o.  .A  /OX/>
  /...     \\X8OOOXX/|OOOOO|OOOOO8X\\X/ <_
 888888b    \\XXX8X/:|OOOOO|\\8XXXXX      \\
d88888888    |\\X8/::|8XXXX|\\IXX~  \\ d8.  \\
888888888b   \\ \\/ ::lXXXXXl ~~ \\--_|888   \\
Y88DR88888    |  .::'-----'    /_-_|~8~   |
 888888888b   \\  :::          /_ -_|      |
 8888GUZ888    | ::          /_ -_ |      |
  888888888b   \\:::         /_ -_ /       [
   888888888    |`:        /  -_ /       ]
    88888888b     |       |     /        |
     88888888b    [       |    (        /
      88888888   ]        |            /
       ~888888b  |         \\         ,'
         ~88888  |          `.___,--'
           ~~88_/
";

pub const STAGE_6: &str = "
  +---+
  |   |
  |   
  |   
  |   
  |   
=========
";

pub const STAGE_5: &str = "
  +---+
  |   |
  |   O
  |   
  |   
  |   
=========
";

pub const STAGE_4: &str = "
  +---+
  |   |
  |   O
  |   |
  |   
  |   
=========
";

pub const STAGE_3: &str = "
  +---+
  |   |
  |   O
  |  /|
  |   
  |   
=========
";

pub const STAGE_2: &str = "
  +---+
  |   |
  |   O
  |  /|\\
  |   
  |   
=========
";

pub const STAGE_1: &str = "
  +---+
  |   |
  |   O
  |  /|\\
  |  / 
  |   
=========
";

pub const STAGE_0: &str = "
  +---+
  |   |
  |   O
  |  /|\\
  |  / \\
  |   
=========
";

pub const INFO_DICT: &'static [(u8, &'static str)] = &[
    (1, "Presione Enter para iniciar el juego, I para ver las instrucciones, A para entrar en los ajustes o E para salir..."),
    (2, "Presione Enter para continuar..."),
    (3, "Ingrese la palabra a adivinar: "),
    (4, "La palabra debe tener al menos 2 letras."),
    (5, "La palabra debe tener menos de 15 letras."),
    (6, "¡Felicidades! Has encontrado el huevo de pascua."),
    (7, "Prueba una letra"),
    (8, "Por favor, ingrese un carácter válido."),
    (9, "Letra ya usada."),
    (10, "Letra incorrecta."),
    (11, "Letra aceptada."),
    (12, "G A M E   O V E R!!!\nLa palabra era: "),
    (13, "F E L I C I D A D E S!!!\nHas adivinado la palabra: "),
    (14, "Vidas: "),
    (15, "No se pudo leer la línea."),
    (16, "Por favor, seleccione una opción válida"),
    (17, "Desea volver a intentarlo? (s/n): "),
    (18, "
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⡀⠀⠀⢀⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣤⣤⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣿⣿⣿⣿⣿⣿⣿⣿⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⢿⣿⣿⣿⣿⣿⣿⡿⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⢀⣀⣠⠀⣶⣤⣄⣉⣉⣉⣉⣠⣤⣶⠀⣄⣀⡀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⣶⣾⣿⣿⣿⣿⣦⣄⣉⣙⣛⣛⣛⣛⣋⣉⣠⣴⣿⣿⣿⣿⣷⣶⠀⠀⠀
⠀⠀⠀⠀⠈⠉⠉⠛⠛⠛⠻⠿⠿⠿⠿⠿⠿⠿⠿⠟⠛⠛⠛⠉⠉⠁⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⣷⣆⠀⠀⠀⢠⡄⠀⠀⠀⣰⣾⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⢀⣠⣶⣾⣿⡆⠸⣿⣶⣶⣾⣿⣿⣷⣶⣶⣿⠇⢰⣿⣷⣶⣄⡀⠀⠀⠀
⠀⠀⠺⠿⣿⣿⣿⣿⣿⣄⠙⢿⣿⣿⣿⣿⣿⣿⡿⠋⣠⣿⣿⣿⣿⣿⠿⠗⠀⠀
⠀⠀⠀⠀⠀⠙⠻⣿⣿⣿⣷⡄⠈⠙⠛⠛⠋⠁⢠⣾⣿⣿⣿⠟⠋⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⣀⣤⣬⣿⣿⣿⣇⠐⣿⣿⣿⣿⠂⣸⣿⣿⣿⣥⣤⣀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠘⠻⠿⠿⢿⣿⣿⣿⣧⠈⠿⠿⠁⣼⣿⣿⣿⡿⠿⠿⠟⠃⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⢿⠀⣶⣦⠀⡿⠛⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠛⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    Inserte contraseña: "),
    (19, "Acceso concedido"),
    (20, "Acceso denegado"),
    (21, "Presione Enter para continuar...?"),
    (22, "
*******************************************************
*                                                     *
*              ¡BIENVENIDO AL JUEGO DEL               *
*                      AHORCADO!                      *
*                     ________                        *
*                     |/      |                       *
*                     |      (_)                      *
*                     |      \\|/                      *
*                     |       |                       *
*                     |      / \\                      *
*                     |                               *
*                    _|___                            *
*                                                     *
*******************************************************
"),
(23, "
**********************************************************************************************
*                                      Instrucciones:                                        *
*                                                                                            *
* - Debes adivinar la palabra oculta.                                                        *
* - Tienes un número limitado de vidas (por defecto es 4, puedes cambiarlo en ajustes).      *
* - Cada vez que ingreses una letra incorrecta, perderás una vida.                           *
* - Si adivinas la palabra sin perder todas tus vidas, ¡felicidades!                         *
* - Si te quedas sin vidas, pierdes.                                                         *
* - ¡Diviértete!                                                                             *
*                                                                                            *
* El juego aún está en desarrollo, así que ignora cualquier error (^_^')                     *
*                                                                                            *
* Recuerda explorar todas las partes del juego, a veces hay sorpresas ocultas...             *
* Como un 'easteregg'... esperando ser descubierto.                                          *
**********************************************************************************************
"),
(24, "
**************************************************************************
*                                 Ajustes:                               *
*                                                                        *
* - Presiona 1 para cambiar el color.                                    *
* - Presiona 2 para alternar la musica.                                  *
* - Presiona 3 para cambiar el numero de jugadores.                      *
* - Presiona 4 para cambiar el idioma.                                   *
* - Presiona 5 para cambiar la dificultad.                               *
* - Presiona cualquier otra cosa para para volver al menu principal.     *
* - Me parece que puede existir algo 'oculto' en alguna parte...         *
**************************************************************************
"),
(25, "
************************************************************************
*                                  Idioma:                             *
*                                                                      *
* 1.  Español (ES).                                                    *
* 2.  English (EN).                                                    *
************************************************************************
"),
(26, "
**********************************
*                                 *
*   Seleccione la dificultad:     *
*                                 *
*   1. Facil (6 vidas)   (^o^)    *
*   2. Medio (4 vidas)   (·_·')   *
*   3. Dificil (2 vidas)  O_O     *
*   4. Imposible (1 vidas) x_x    *
*                                 *
**********************************
"),
(27, "¡Felicidades! Has encontrado el segundo huevo de pascua. Que suerte tienes ;) 🍀"),
(28, "
************************************************************************
*                                Jugadores:                            *
*                                                                      *
* 1.  Un solo jugador.                                                 *
* 2.  2 Jugadores (no implementado, rompera el juego).                 *
************************************************************************
"),
(29, "Por favor elige el número de jugadores"),
(30, "Palabra: "),
(31, "Letras adivinadas: ")];

pub const INFO_DICTEN: &'static [(u8, &'static str)] = &[
    (
        1,
        "Press Enter to start the game, I to see the instructions, S to enter the settings, or E to exit...",
    ),
    (2, "Press Enter to continue ..."),
    (3, "Enter the word to guess:"),
    (4, "The word must have at least 2 letters."),
    (5, "The word must have less than 15 letters."),
    (6, "Congratulations! You have found the Easter egg."),
    (7, "Try a letter"),
    (8, "Please enter a valid character."),
    (9, "Letter already used."),
    (10, "Incorrect letter."),
    (11, "Accepted letter."),
    (12, "G A M E   O V E R!!!\nThe word was: "),
    (
        13,
        "C O N G R A T U L A T I O N S!!!\nYou have guessed the word: ",
    ),
    (14, "Lives: "),
    (15, "The line could not be read."),
    (16, "Please select a valid option"),
    (17, "Do you want to try again? (y/n): "),
    (18, "
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⡀⠀⠀⢀⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣤⣤⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣿⣿⣿⣿⣿⣿⣿⣿⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⢿⣿⣿⣿⣿⣿⣿⡿⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⢀⣀⣠⠀⣶⣤⣄⣉⣉⣉⣉⣠⣤⣶⠀⣄⣀⡀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⣶⣾⣿⣿⣿⣿⣦⣄⣉⣙⣛⣛⣛⣛⣋⣉⣠⣴⣿⣿⣿⣿⣷⣶⠀⠀⠀
⠀⠀⠀⠀⠈⠉⠉⠛⠛⠛⠻⠿⠿⠿⠿⠿⠿⠿⠿⠟⠛⠛⠛⠉⠉⠁⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⣷⣆⠀⠀⠀⢠⡄⠀⠀⠀⣰⣾⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⢀⣠⣶⣾⣿⡆⠸⣿⣶⣶⣾⣿⣿⣷⣶⣶⣿⠇⢰⣿⣷⣶⣄⡀⠀⠀⠀
⠀⠀⠺⠿⣿⣿⣿⣿⣿⣄⠙⢿⣿⣿⣿⣿⣿⣿⡿⠋⣠⣿⣿⣿⣿⣿⠿⠗⠀⠀
⠀⠀⠀⠀⠀⠙⠻⣿⣿⣿⣷⡄⠈⠙⠛⠛⠋⠁⢠⣾⣿⣿⣿⠟⠋⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⣀⣤⣬⣿⣿⣿⣇⠐⣿⣿⣿⣿⠂⣸⣿⣿⣿⣥⣤⣀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠘⠻⠿⠿⢿⣿⣿⣿⣧⠈⠿⠿⠁⣼⣿⣿⣿⡿⠿⠿⠟⠃⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⢿⠀⣶⣦⠀⡿⠛⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠛⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    Insert password:"),
    (19, "Access granted"),
    (20, "Access denied"),
    (21, "Press Enter to continue...?"),
    (
        22,
        "
*******************************************************
*                                                     *
*                WELCOME TO THE HANGMAN               *
*                        GAME!                        *
*                     ________                        *
*                     |/      |                       *
*                     |      (_)                      *
*                     |      \\|/                      *
*                     |       |                       *
*                     |      / \\                      *
*                     |                               *
*                    _|___                            *
*                                                     *
*******************************************************
",
    ),
    (
        23,
        "
**********************************************************************************************
*                                       Instructions:                                        *
*                                                                                            *
* - You must guess the hidden word.                                                          *
* - You have a limited number of lives (default is 4, you can change it in settings).        *
* - Every time you enter an incorrect letter, you will lose a life.                          *
* - If you guess the word without losing all your lives, congrats!                           *
* - If you run out of lives, you lose.                                                       *
* - Have fun!                                                                                *
*                                                                                            *
* The game is still under development, so please ignore any bugs (^_^')                      *
*                                                                                            *
* Remember to explore all parts of the game, sometimes there are hidden surprises...         *
* Like an 'easteregg'... just waiting to be found.                                           *
**********************************************************************************************
",
    ),
    (
        24,
        "
**************************************************************************
*                                 Settings:                              *
*                                                                        *
* - Press 1 to change the color.                                         *
* - Press 2 to toggle the music.                                         *
* - Press 3 to change the number of players.                             *
* - Press 4 to change the language.                                      *
* - Press 5 to change the difficulty.                                    *
* - Press anything else to return to the main menu.                      *
* - It seems to me that there may be something 'hidden' somewhere...     *
**************************************************************************
",
    ),
    (
        25,
        "
************************************************************************
*                               Language:                              *
*                                                                      *
* 1.  Español (ES).                                                    *
* 2.  English (EN).                                                    *
************************************************************************
",
    ),
    (
        26,
        "
**********************************
*                                 *
*   Select the difficulty:        *
*                                 *
*   1. Easy (6 lives)     (^o^)   *
*   2. Medium (4 lives)   (·_·')  *
*   3. Hard (2 lives)      O_O    *
*   4. Impossible (1 lives) x_x   *
*                                 *
**********************************
",
    ),
    (
        27,
        "Congratulations! You have found the second Easter egg. How lucky you are ;) 🍀",
    ),
    (
        28,
        "
    ************************************************************************
    *                                Players:                              *
    *                                                                      *
    * 1.  Single player.                                                   *
    * 2.  2 Players (Not implemented, will break the game).                *
    ************************************************************************
    ",
    ),
    (29, "Please choose the number of players"),
    (30, "Word: "),
    (31, "Guessed letters: ")
];

pub const WORDS_DICT: &'static [&str] = &[
    "amor",
    "felicidad",
    "luz",
    "montaña",
    "estrella",
    "corazon",
    "viento",
    "sol",
    "libertad",
    "cielo",
    "esperanza",
    "fuerza",
    "paz",
    "union",
    "misterio",
    "infinito",
    "noche",
    "mar",
    "eternidad",
    "fuego",
    "raiz",
    "jardin",
    "palabra",
    "nieve",
    "futuro",
    "pasado",
    "presente",
    "alma",
    "mente",
    "cuerpo",
    "fruta",
    "puente",
    "camino",
    "tristeza",
    "alegria",
    "luna",
    "risa",
    "magia",
    "fantasia",
    "realidad",
    "sueño",
    "brillante",
    "oro",
    "silencio",
    "musica",
    "ruido",
    "amistad",
    "trabajo",
    "familia",
    "vida",
    "arbol",
    "rio",
    "montana",
    "casa",
    "libro",
    "juego",
    "arte",
    "ciencia",
    "historia",
    "misterio",
    "aventura",
    "tierra",
    "futuro",
    "pasado",
    "cultura",
    "naturaleza",
    "planeta",
    "galaxia",
    "universo",
    "ciudad",
    "pueblo",
    "bosque",
    "mariposa",
    "flor",
    "lluvia",
    "nube",
    "sol",
    "noche",
    "dia",
    "relampago",
    "tormenta",
    "verano",
    "invierno",
    "primavera",
    "otoño",
    "fresco",
    "calor",
    "frio",
    "neblina",
    "nieve",
    "granizo",
    "tempestad",
    "brisa",
    "huracan",
    "ciclon",
    "terremoto",
    "volcan",
    "desierto",
    "playa",
    "isla",
    "oceano",
    "mar",
    "lago",
    "rio",
    "fuente",
];

pub const WORDS_DICTEN: &'static [&str] = &[
    "love",
    "happiness",
    "light",
    "mountain",
    "star",
    "heart",
    "wind",
    "sun",
    "freedom",
    "sky",
    "hope",
    "strength",
    "peace",
    "union",
    "mystery",
    "infinite",
    "night",
    "sea",
    "eternity",
    "fire",
    "root",
    "garden",
    "word",
    "snow",
    "future",
    "past",
    "present",
    "soul",
    "mind",
    "body",
    "fruit",
    "bridge",
    "path",
    "sadness",
    "joy",
    "moon",
    "laughter",
    "magic",
    "fantasy",
    "reality",
    "dream",
    "bright",
    "gold",
    "silence",
    "music",
    "noise",
    "friendship",
    "work",
    "family",
    "life",
    "tree",
    "river",
    "mountain",
    "house",
    "book",
    "game",
    "art",
    "science",
    "history",
    "mystery",
    "adventure",
    "earth",
    "future",
    "past",
    "culture",
    "nature",
    "planet",
    "galaxy",
    "universe",
    "city",
    "town",
    "forest",
    "butterfly",
    "flower",
    "rain",
    "cloud",
    "sun",
    "night",
    "day",
    "lightning",
    "storm",
    "summer",
    "winter",
    "spring",
    "autumn",
    "cool",
    "heat",
    "cold",
    "fog",
    "snow",
    "hail",
    "tempest",
    "breeze",
    "hurricane",
    "cyclone",
    "earthquake",
    "volcano",
    "desert",
    "beach",
    "island",
    "ocean",
    "sea",
    "lake",
    "river",
    "fountain",
];
