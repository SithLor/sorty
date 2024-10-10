import java.util.Scanner;

class Main {
    public static void main(String[] args) {
        System.out.println("Hello, World!");
        Scanner scanner = new Scanner(System.in);
        
        // Listen to terminal for fake controller input
        while (true) {
            System.out.println("Enter motor speed (0-100) or 'exit' to quit:");
            String input = scanner.nextLine();
            
            if (input.equalsIgnoreCase("exit")) {
                break;
            }
            
            try {
                int speed = Integer.parseInt(input);
                set_motor_speed(speed);
                steper_motor(speed, 10);
            } catch (NumberFormatException e) {
                System.out.println("Invalid input. Please enter a number between 0 and 100.");
            }
        }
        
        scanner.close();
    }

    // This stub is for motor control
    public static void set_motor_speed(int speed) {
        if (speed < 0 || speed > 100) {
            throw new IllegalArgumentException("Speed must be between 0 and 100");
        }
        // Code to set motor speed
        System.out.println("Motor speed set to " + speed);
    }

    // Take input 0-100
    public static void steper_motor(int input, int step_speed) {
        if (input < 0 || input > 100) {
            throw new IllegalArgumentException("Input must be between 0 and 100");
        }
        if (step_speed < 0) {
            throw new IllegalArgumentException("Step speed must be non-negative");
        }

        // Simulate slow ramp up
        for (int i = 0; i <= input; i += step_speed) {
            System.out.println("Stepper motor speed: " + i);
            try {
                Thread.sleep(100); // Simulate time delay for ramp up
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
            }
        }

        // Simulate slow ramp down
        for (int i = input; i >= 0; i -= step_speed) {
            System.out.println("Stepper motor speed: " + i);
            try {
                Thread.sleep(100); // Simulate time delay for ramp down
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
            }
        }
    }
}