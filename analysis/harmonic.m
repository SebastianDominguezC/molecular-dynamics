clear all; clc;

x = load("./harmonic/x.csv");
y = load("./harmonic/y.csv");

t = 20.0;
dt = 0.0001;
steps = t / dt;

L = 5;
figure(1);
scat = scatter(x(1, :), y(1, :));
xlim([-L, L]);
ylim([-L, L]);
xlabel('x');
ylabel('y');
grid on;


s = 10000;

for i = s * 1 : s: steps
        scat.XData = x(i, :);
        scat.YData = y(i, :);
        pause(0.1);
end

disp('finito');

figure(2);
plot(x, y);

prom_x = x - mean(x);
x_auto = xcorr(prom_x, prom_x);
prom_y = y - mean(y);
y_auto = xcorr(prom_y, prom_y);
desfase = xcorr(prom_x, prom_y);

figure(3);
hold on;
plot(x_auto);
plot(y_auto);
plot(desfase);
legend('Auto X', 'Auto Y', 'Cross XY');

