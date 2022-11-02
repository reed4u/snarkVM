// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use super::*;

impl<N: Network, C: ConsensusStorage<N>> VM<N, C> {
    /// Finalizes the transaction into the VM.
    /// This method assumes the given transaction **is valid**.
    #[inline]
    pub fn finalize(&mut self, transaction: &Transaction<N>) -> Result<()> {
        // Ensure the transaction is valid.
        ensure!(self.verify(transaction), "Invalid transaction: failed to verify");
        // Finalize the transaction.
        match transaction {
            Transaction::Deploy(_, deployment, _) => self.finalize_deployment(deployment),
            Transaction::Execute(_, execution, _) => self.finalize_execution(execution),
        }
    }

    /// Finalizes the deployment in the VM.
    /// This method assumes the given deployment **is valid**.
    #[inline]
    fn finalize_deployment(&mut self, deployment: &Deployment<N>) -> Result<()> {
        self.process.write().finalize_deployment::<C::ProgramStorage>(self.program_store(), deployment)
    }

    /// Finalizes the execution in the VM.
    /// This method assumes the given execution **is valid**.
    #[inline]
    fn finalize_execution(&mut self, execution: &Execution<N>) -> Result<()> {
        self.process.write().finalize_execution::<C::ProgramStorage>(self.program_store(), execution)
    }
}

#[cfg(test)]
mod tests {
    use crate::vm::test_helpers::sample_program;
    use snarkvm_utilities::TestRng;

    #[test]
    fn test_finalize() {
        let rng = &mut TestRng::default();

        let mut vm = crate::vm::test_helpers::sample_vm();

        // Fetch a deployment transaction.
        let deployment_transaction = crate::vm::test_helpers::sample_deployment_transaction(rng);

        // Finalize the transaction.
        vm.finalize(&deployment_transaction).unwrap();

        // Ensure the VM can't redeploy the same transaction.
        assert!(vm.finalize(&deployment_transaction).is_err());
    }

    #[test]
    fn test_finalize_deployment() {
        let rng = &mut TestRng::default();
        let mut vm = crate::vm::test_helpers::sample_vm();

        // Fetch the program from the deployment.
        let program = sample_program();

        // Deploy the program.
        let deployment = vm.deploy(&program, rng).unwrap();

        // Ensure the program does not exists.
        assert!(!vm.contains_program(program.id()));

        // Finalize the deployment.
        vm.finalize_deployment(&deployment).unwrap();

        // Ensure the program exists.
        assert!(vm.contains_program(program.id()));
    }
}
