import React, { useEffect, useState } from 'react';
import { Bar } from 'react-chartjs-2';
import { 
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  BarElement,
  Title,
  Tooltip,
  Legend
} from 'chart.js';

ChartJS.register(
  CategoryScale,
  LinearScale,
  BarElement,
  Title,
  Tooltip,
  Legend
);

interface DiskUsage {
  nome: string;
  uso: number;
  total: number;
  disponivel: number;
}

interface ThreadMetrics {
  total_threads: number;
  active_threads: number;
  thread_per_core: number;
}

interface MetricsData {
  timestamp: number;
  cpu_usage: number;
  total_memory_gb: number;
  used_memory_gb: number;
  disk_usage: DiskUsage[];
  thread_metrics: ThreadMetrics;
}

const MetricsDisplay: React.FC = () => {
  const [currentMetric, setCurrentMetric] = useState<MetricsData | null>(null);
  const [error, setError] = useState<string>('');
  
  useEffect(() => {
    const connectWebSocket = () => {
      const ws = new WebSocket('ws://localhost:3030/ws');
      
      ws.onopen = () => {
        console.log('Conexão WebSocket estabelecida');
        setError('');
      };
      
      ws.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);
          console.log('Dados recebidos:', data);
          
          if (data && typeof data === 'object') {
            const GB = 1024 * 1024 * 1024;
            const processedData: MetricsData = {
              timestamp: Date.now(),
              cpu_usage: Number(data.cpu_usage) || 0,
              total_memory_gb: Number(data.total_memory) / GB || 0,
              used_memory_gb: Number(data.used_memory) / GB || 0,
              disk_usage: data.disk_usage?.map((disk: any) => ({
                nome: disk.nome,
                uso: Number(disk.uso) || 0,
                total: Number(disk.total) / GB || 0,
                disponivel: Number(disk.disponivel) / GB || 0
              })) || [],
              thread_metrics: {
                total_threads: Number(data.thread_metrics?.total_threads) || 0,
                active_threads: Number(data.thread_metrics?.active_threads) || 0,
                thread_per_core: Number(data.thread_metrics?.thread_per_core) || 0
              }
            };

            console.log('Dados processados:', processedData);
            setCurrentMetric(processedData);
          }
        } catch (err) {
          console.error('Erro ao processar dados:', err);
          console.error('Dados recebidos:', event.data);
          setError('Erro ao processar dados do servidor');
        }
      };
      
      ws.onerror = (event) => {
        console.error('Erro na conexão WebSocket:', event);
        setError('Erro na conexão com o servidor. Tentando reconectar...');
        setTimeout(connectWebSocket, 3000);
      };
      
      ws.onclose = () => {
        console.log('Conexão WebSocket fechada');
        setError('Conexão perdida. Tentando reconectar...');
        setTimeout(connectWebSocket, 3000);
      };

      return ws;
    };

    const ws = connectWebSocket();
    
    return () => {
      if (ws) {
        ws.close();
      }
    };
  }, []);
  
  const chartData = {
    labels: ['CPU', 'Memória', 'SSD', 'HD', 'Threads'],
    datasets: [{
      label: 'Métricas do Sistema',
      data: currentMetric ? [
        currentMetric.cpu_usage || 0,
        currentMetric.total_memory_gb > 0 
          ? Number(((currentMetric.used_memory_gb / currentMetric.total_memory_gb) * 100).toFixed(2))
          : 0,
        Number(currentMetric.disk_usage?.[0]?.uso || 0),
        Number(currentMetric.disk_usage?.[1]?.uso || 0),
        currentMetric.thread_metrics?.total_threads > 0
          ? Number(((currentMetric.thread_metrics.active_threads / currentMetric.thread_metrics.total_threads) * 100).toFixed(2))
          : 0
      ] : [0, 0, 0, 0, 0],
      backgroundColor: [
        'rgba(75, 192, 192, 0.5)',
        'rgba(255, 99, 132, 0.5)',
        'rgba(53, 162, 235, 0.5)',
        'rgba(153, 102, 255, 0.5)',
        'rgba(255, 206, 86, 0.5)'
      ],
      borderColor: [
        'rgb(75, 192, 192)',
        'rgb(255, 99, 132)',
        'rgb(53, 162, 235)',
        'rgb(153, 102, 255)',
        'rgb(255, 206, 86)'
      ],
      borderWidth: 1
    }]
  };
  
  const renderMetricsDetails = () => {
    if (!currentMetric) return null;

    return (
      <div className="metrics-details">
        <p>CPU: {currentMetric.cpu_usage?.toFixed(2)}%</p>
        <p>
          Memória: {currentMetric.used_memory_gb?.toFixed(2)} / {currentMetric.total_memory_gb?.toFixed(2)} GB 
          ({((currentMetric.used_memory_gb / currentMetric.total_memory_gb) * 100).toFixed(2)}%)
        </p>
        
        <div style={{ marginTop: '10px' }}>
          <h4>Armazenamento:</h4>
          {currentMetric.disk_usage?.map((disk, index) => (
            <p key={disk.nome || index}>
              {disk.nome}: {disk.uso?.toFixed(2)}% utilizado
              ({((disk.total - disk.disponivel)/1).toFixed(2)}GB / {(disk.total/1).toFixed(2)}GB)
            </p>
          ))}
        </div>
        
        <div>
          <p>
            Threads Ativos: {currentMetric.thread_metrics?.active_threads}/
            {currentMetric.thread_metrics?.total_threads}
          </p>
          <p>
            Threads por Core: {currentMetric.thread_metrics?.thread_per_core?.toFixed(2)}
          </p>
        </div>
      </div>
    );
  };

  return (
    <div className="metrics-display">
      <h2>Monitor de Métricas do Sistema em Tempo Real</h2>
      {error && (
        <div className="error-message" style={{ color: 'red', margin: '10px 0' }}>
          {error}
        </div>
      )}
      <div style={{ height: '400px', width: '100%', marginBottom: '20px' }}>
        <Bar 
          data={chartData} 
          options={{
            animation: {
              duration: 300
            },
            scales: {
              y: {
                beginAtZero: true,
                max: 100,
                title: {
                  display: true,
                  text: 'Porcentagem (%)'
                }
              }
            },
            responsive: true,
            maintainAspectRatio: false,
            plugins: {
              legend: {
                display: true,
                position: 'top' as const
              },
              tooltip: {
                callbacks: {
                  label: function(context: any) {
                    try {
                      if (!context?.parsed?.y) {
                        return '';
                      }

                      const value = Number(context.parsed.y) || 0;
                      const label = context.label || '';

                      if (label === 'Memória' && currentMetric) {
                        const usedMem = currentMetric.used_memory_gb || 0;
                        const totalMem = currentMetric.total_memory_gb || 1;
                        return `Memória: ${value.toFixed(2)}% (${usedMem.toFixed(2)}GB/${totalMem.toFixed(2)}GB)`;
                      }

                      if ((label === 'SSD' || label === 'HD') && currentMetric?.disk_usage) {
                        const diskIndex = label === 'SSD' ? 0 : 1;
                        const disk = currentMetric.disk_usage[diskIndex];
                        if (disk) {
                          const used = disk.total - disk.disponivel;
                          const total = disk.total;
                          return `${label}: ${value.toFixed(2)}% (${used.toFixed(2)}GB/${total.toFixed(2)}GB)`;
                        }
                        return `${label}: ${value.toFixed(2)}%`;
                      }

                      if (label === 'Threads' && currentMetric?.thread_metrics) {
                        const activeThreads = String(currentMetric.thread_metrics.active_threads);
                        const totalThreads = String(currentMetric.thread_metrics.total_threads);
                        return `Threads: ${value.toFixed(2)}% (${activeThreads}/${totalThreads})`;
                      }

                      return `CPU: ${value.toFixed(2)}%`;
                    } catch (err) {
                      console.error('Erro no tooltip:', err);
                      return '';
                    }
                  }
                }
              }
            }
          }} 
        />
      </div>
      {renderMetricsDetails()}
    </div>
  );
};

export default MetricsDisplay;