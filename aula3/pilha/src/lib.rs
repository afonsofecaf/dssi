// Definição da estrutura de Pilha (Stack)
pub struct Stack<T> {
    // Vetor que armazena os elementos da pilha
    elementos: Vec<T>,
    // Capacidade máxima da pilha (opcional)
    capacidade_maxima: Option<usize>,
}

// Implementação dos métodos para a estrutura Stack
impl<T> Stack<T> {
    // Método para criar uma nova pilha sem limite de capacidade
    pub fn nova() -> Self {
        Stack {
            elementos: Vec::new(),
            capacidade_maxima: None,
        }
    }

    // Método para verificar se a pilha está vazia
    pub fn esta_vazia(&self) -> bool {
        self.elementos.is_empty()
    }

    // Método para criar uma nova pilha com capacidade máxima
    pub fn nova_com_capacidade(capacidade: usize) -> Self {
        Stack {
            elementos: Vec::with_capacity(capacidade),
            capacidade_maxima: Some(capacidade),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nova_pilha_esta_vazia() {
        let pilha: Stack<i32> = Stack::nova();
        assert!(pilha.esta_vazia());
    }

    #[test]
    fn nova_pilha_com_capacidade() {
        let pilha: Stack<i32> = Stack::nova_com_capacidade(5);
        assert!(pilha.esta_vazia());
        assert_eq!(pilha.tamanho(), 0);
    }
}