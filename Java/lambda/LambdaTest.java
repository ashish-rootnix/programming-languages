package lambda;

import java.util.*;
import javax.swing.*;
import javax.swing.Timer;

/**
 * This program demonstrate the use of lambda expressions.
 */

public class LambdaTest {
    public static void main(String[] args)
    {
        var plannets = new String[] {"Mercury", "Venus", "Earth", "Mars", "Jupitor",
                                     "Saturn", "Uranus", "Neptune"};
        System.out.println(Arrays.toString(plannets));
        
        System.out.println("Sorted in dictionary order");
        Arrays.sort(plannets);
        System.out.println(Arrays.toString(plannets));
        
        System.out.println("Sorted by length");
        Arrays.sort(plannets, (first, second)->first.length()-second.length());
        System.out.println(Arrays.toString(plannets));

        var timer = new Timer(1000, event -> System.out.println("The time is "+ new Date()));
        timer.start();

        JOptionPane.showMessageDialog(null, "Quit Program?");
        System.exit(0);
    }
}

/*
Output:
    [Mercury, Venus, Earth, Mars, Jupitor, Saturn, Uranus, Neptune]
    Sorted in dictionary order
    [Earth, Jupitor, Mars, Mercury, Neptune, Saturn, Uranus, Venus]
    Sorted by length
    [Mars, Earth, Venus, Saturn, Uranus, Jupitor, Mercury, Neptune]
    The time is Tue Feb 17 14:07:06 IST 2026
    The time is Tue Feb 17 14:07:07 IST 2026
    The time is Tue Feb 17 14:07:08 IST 2026
    The time is Tue Feb 17 14:07:09 IST 2026
    The time is Tue Feb 17 14:07:10 IST 2026
    The time is Tue Feb 17 14:07:11 IST 2026
    The time is Tue Feb 17 14:07:12 IST 2026
    The time is Tue Feb 17 14:07:13 IST 2026
*/
