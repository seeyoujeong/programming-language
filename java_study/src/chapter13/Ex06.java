package chapter13;
import java.awt.*;
import java.awt.event.*;
import javax.swing.*;

class RandomThread extends Thread {
	private Container contentPane;
	private boolean flag=false;
	
	public RandomThread(Container contentPane) {
		this.contentPane = contentPane;
	}
	public void finish() {
		flag = true;
	}
	
	@Override
	public void run() {
		while(true) {
			int x = ((int)(Math.random()*contentPane.getWidth()));
			int y = ((int)(Math.random()*contentPane.getHeight()));
			JLabel label = new JLabel("java");
			label.setSize(80, 30);
			label.setLocation(x, y);
			contentPane.add(label);
			contentPane.repaint();
			try {
				Thread.sleep(300);
				if(flag == true) {
					contentPane.removeAll();
					label = new JLabel("finish");
					label.setSize(80, 30);
					label.setLocation(100, 100);
					
					label.setForeground(Color.RED);
					contentPane.add(label);
					contentPane.repaint();
					return;
				}
			}
			catch(InterruptedException e) { return; }
		}
	}
}
public class Ex06 extends JFrame {
	private RandomThread th;
	
	public Ex06() {
		setTitle("ThreadFinishFlagEx ����");
		setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
		Container c = getContentPane();
		c.setLayout(null);
		
		c.addMouseListener(new MouseAdapter() {
			@Override
			public void mousePressed(MouseEvent e) {
				th.finish();
			}
		});
		
		setSize(300,200);
		setVisible(true);
		
		th = new RandomThread(c);
		th.start();
	}

	public static void main(String[] args) {
		new Ex06();

	}

}
