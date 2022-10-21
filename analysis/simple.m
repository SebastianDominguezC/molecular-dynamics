clear all; clc;

% Load data here
x = load("./sim1/x.csv");
y = load("./sim1/y.csv");
z = load("./sim1/z.csv");

t = 1.0;
dt = 0.00001;
steps = t / dt;

L = 10;

figure(1);
scat = plot3(x(1, :), y(1, :), z(1, :), 'o', 'MarkerSize', 5,'MarkerFaceColor','#D9FFFF');
xlim([-L, L]);
ylim([-L, L]);
zlim([-L, L]);
xlabel('x');
ylabel('y');
zlabel('z');
grid on;

angle = 0;
view(angle, 45);

s = 10;

for i = s : s: steps
        scat.XData = x(i, :);
        scat.YData = y(i, :);
        scat.ZData = z(i, :);
        view(angle, 45);
        pause(0.001);
        angle = angle + 0.05;
end

disp('finito');