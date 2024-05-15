package org.example;

import java.util.ArrayList;

public class MiClase extends Clase{
    private String name;

    public MiClase(String name){
        this.name = name;
    }

    public static void main(String[] args) {
        System.out.println("Hola, mundo!");
    }

    public String getName(){
        return this.name;
    }

    public String[] lista(){
        return new String[]{this.name + "lista"};
    }

    public ArrayList<Integer> sizeName(){
        return new ArrayList<>(name.length());
    }
}

