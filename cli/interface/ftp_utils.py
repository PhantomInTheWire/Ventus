# Console utility class for printing colors and displaying ASCII art
class ConsoleUtils:
    # Color codes
    COLORS = {
        'red': "\033[1;31m",
        'green': "\033[1;32m",
        'yellow': "\033[1;33m",
        'blue': "\033[1;34m",
        'cyan': "\033[1;36m",
        'purple': "\033[1;35m",
        'reset': "\033[0m",
        'bcyan': "\033[1;96m",
        'bgreen': "\033[1;92m",
    }

    def display_ascii_art(self):
        """Displays the FTP server banner ASCII art with color gradients."""
        art = r"""
██╗   ██╗███████╗███╗   ██╗████████╗██╗   ██╗███████╗
██║   ██║██╔════╝████╗  ██║╚══██╔══╝██║   ██║██╔════╝
██║   ██║█████╗  ██╔██╗ ██║   ██║   ██║   ██║███████╗
╚██╗ ██╔╝██╔══╝  ██║╚██╗██║   ██║   ██║   ██║╚════██║
 ╚████╔╝ ███████╗██║ ╚████║   ██║   ╚██████╔╝███████║
  ╚═══╝  ╚══════╝╚═╝  ╚═══╝   ╚═╝    ╚═════╝ ╚══════╝
        """
        print(self.COLORS['red'] + art + self.COLORS['reset'])

    def print_with_color(self, message, color='reset'):
        """Prints a message with the specified color."""
        print(self.COLORS.get(color, self.COLORS['reset']) + message + self.COLORS['reset'])

