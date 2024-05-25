import java.util.ArrayList;
import java.util.List;

abstract class Shape {
    // Native methods
    public native double calculateCircleArea(double radius);

    public native double calculateCirclePerimeter(double radius);

    public native double calculateArea(double side);

    public native double calculatePerimeter(double side);

    public native double calculateArea1(double side1, double side2);

    public native double calculatePerimeter1(double side1, double side2);

    public native double calculateArea2(double side1, double side2, double side3);

    public native double calculatePerimeter2(double side1, double side2, double side3);

    protected String name;

    // Abstract methods
    public Shape(String name) {
        this.name = name;
    }

    public abstract double calculateArea();

    public abstract double calculatePerimeter();

    public String getName() {
        return name;
    }

    static {
        System.loadLibrary("shape");
    }
}

class Circle extends Shape {
    private double radius;

    public Circle(double radius) {
        super("Circle");
        this.radius = radius;
    }

    public double calculateArea() {
        return super.calculateCircleArea(radius);
    }

    public double calculatePerimeter() {
        return super.calculateCirclePerimeter(radius);
    }
}

class Square extends Shape {
    private double side;

    public Square(double side) {
        super("Square");
        this.side = side;
    }

    public double calculateArea() {
        return super.calculateArea(side);
    }

    public double calculatePerimeter() {
        return super.calculatePerimeter(side);
    }
}

class Rectangle extends Shape {
    private double side1, side2;

    public Rectangle(double side1, double side2) {
        super("Rectangle");
        this.side1 = side1;
        this.side2 = side2;
    }

    public double calculateArea() {
        return super.calculateArea1(side1, side2);
    }

    public double calculatePerimeter() {
        return super.calculatePerimeter1(side1, side2);
    }
}

class Triangle extends Shape {
    private double side1, side2, side3;

    public Triangle(double side1, double side2, double side3) {
        super("Triangle");
        this.side1 = side1;
        this.side2 = side2;
        this.side3 = side3;
    }

    public double calculateArea() {
        return super.calculateArea2(side1, side2, side3);
    }

    public double calculatePerimeter() {
        return super.calculatePerimeter2(side1, side2, side3);
    }
}

public class JNIShape {
    public static void main(String[] args) {
        Circle circle = new Circle(5);
        Square square = new Square(6);
        Rectangle rectangle = new Rectangle(3, 4);
        Triangle triangle = new Triangle(3, 4, 5);

        List<Shape> shapes = new ArrayList<>();
        shapes.add(circle);
        shapes.add(square);
        shapes.add(rectangle);
        shapes.add(triangle);

        for (Shape shape : shapes) {
            System.out.println("Shape: " + shape.getName());
            System.out.println("Area: " + shape.calculateArea());
            System.out.println("Perimeter: " + shape.calculatePerimeter());
        }
    }
}
