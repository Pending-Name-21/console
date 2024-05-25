import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

class Handler {
    private static native String notifyEvent();

    private static native void helloFromRust();

    public Handler() {
        helloFromRust();
    }

    // private Map<EventType, List<Subscriber>> suscribers = new HashMap<>();
    private Map<String, List<ISubscriber>> suscribers = new HashMap<>();

    public void subscribe(String event, ISubscriber subscriber) {
        suscribers.computeIfAbsent(event, k -> new ArrayList<>()).add(subscriber);
    }

    public void notifySubscribers() {
        String event = Handler.notifyEvent();
        System.out.println("Notifying for event :" + event);
        List<ISubscriber> eventSubscribers = suscribers.get(event);
        if (eventSubscribers != null) {
            for (ISubscriber subscriber : eventSubscribers) {
                subscriber.notify(event);
            }
        }
    }

    public void notifyForSubscribers(String event) {
        System.out.println("Notifying for event :" + event);
        List<ISubscriber> eventSubscribers = suscribers.get(event);
        if (eventSubscribers != null) {
            for (ISubscriber subscriber : eventSubscribers) {
                subscriber.notify(event);
            }
        }
    }

    public ISubscriber[] getSubscribers() {
        List<ISubscriber> allSubscribers = new ArrayList<>();
        for (List<ISubscriber> subs : suscribers.values()) {
            allSubscribers.addAll(subs);
        }
        return allSubscribers.toArray(new ISubscriber[0]);
    }
}

class EventType {
    public static final EventType MOUSE = new EventType("Mouse");
    public static final EventType KEYBOARD = new EventType("Keyboard");

    private String name;

    private EventType(String name) {
        this.name = name;
    }

    public String getName() {
        return name;
    }
}

interface ISubscriber {
    /*
     * public void notify(EventType event)
     */

    public void notify(String eventName);

}

class MouseSubscriber implements ISubscriber {
    public void notify(String eventName) {
        System.out.println("MouseSubscriber notified of event: " + eventName);
    }
}

class KeyboardSubscriber implements ISubscriber {
    public void notify(String eventName) {
        System.out.println("KeyboardSubscriber notified of event: " + eventName);
    }
}

class Movement implements ISubscriber {
    public void notify(String eventName) {
        System.out.println("MovementSubscriber notified of event: " + eventName);
    }
}

class Main {

    static {
        System.loadLibrary("jni_events");
    }

    public void hola() {
        System.out.println("Hola");
    }

    public static void main(String[] args) {
        Handler handler = new Handler();

        ISubscriber mouseSubscriber = new MouseSubscriber();
        ISubscriber keyboardSubscriber = new KeyboardSubscriber();
        ISubscriber movement = new Movement();

        handler.subscribe("Mouse", mouseSubscriber);
        handler.subscribe("Keyboard", keyboardSubscriber);
        handler.subscribe("Mouse", movement);

        handler.notifySubscribers();

        // handler.notifySubscribers("Mouse");
        // handler.notifySubscribers("Keyboard");
    }
}
